# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/USCustomaryUnits
type=file
~~~
# SOURCE
~~~sysml
standard library package <USCU> USCustomaryUnits {
	doc
	/*
	 * Measurement unit declarations generated from NIST SP811 Appendix B
	 *
	 * See https://www.nist.gov/pml/special-publication-811/nist-guide-si-appendix-b-conversion-factors/nist-guide-si-appendix-b8
	 */

    private import MeasurementReferences::*;
    public import ISQ::*;
    private import SI::*;

    attribute 'acre (based on US survey foot)' : AreaUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^2; :>> conversionFactor = 4.046873E+03; :>> isExact = false; } }
    attribute 'acre foot (based on US survey foot)' : VolumeUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^3; :>> conversionFactor = 1.233489E+03; :>> isExact = false; } }
    attribute <bbl> 'barrel (for petroleum, 42 gallons (US))' : VolumeUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^3; :>> conversionFactor = 1.589873E-01; :>> isExact = false; } }
    attribute <Btu_IT> 'British thermal unit (IT)' : EnergyUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = J; :>> conversionFactor = 1.055056E+03; :>> isExact = false; } }
    alias Btu for Btu_IT;
    attribute <Btu_th> 'British thermal unit (th)' : EnergyUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = J; :>> conversionFactor = 1.054350E+03; :>> isExact = false; } }
    attribute <Btu_mean> 'British thermal unit (mean)' : EnergyUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = J; :>> conversionFactor = 1.05587E+03; :>> isExact = false; } }
    attribute <'Btu_39°F'> 'British thermal unit (39 °F)' : EnergyUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = J; :>> conversionFactor = 1.05967E+03; :>> isExact = false; } }
    attribute <'Btu_59°F'> 'British thermal unit (59 °F)' : EnergyUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = J; :>> conversionFactor = 1.05480E+03; :>> isExact = false; } }
    attribute <'Btu_60°F'> 'British thermal unit (60 °F)' : EnergyUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = J; :>> conversionFactor = 1.05468E+03; :>> isExact = false; } }
    attribute <'Btu_IT⋅ft/(h⋅ft²⋅°F)'> 'British thermal unit (IT) foot per hour square foot degree Fahrenheit' : ThermalConductivityUnit = Btu_IT*ft/(h*ft^2*'°F');
    attribute <'Btu_th⋅ft/(h⋅ft²⋅°F)'> 'British thermal unit (th) foot per hour square foot degree Fahrenheit' : ThermalConductivityUnit = Btu_th*ft/(h*ft^2*'°F');
    attribute <'Btu_IT⋅in/(h⋅ft²⋅°F)'> 'British thermal unit (IT) inch per hour square foot degree Fahrenheit' : ThermalConductivityUnit = Btu_IT*'in'/(h*ft^2*'°F');
    attribute <'Btu_th⋅in/(h⋅ft²⋅°F)'> 'British thermal unit (th) inch per hour square foot degree Fahrenheit' : ThermalConductivityUnit = Btu_th*'in'/(h*ft^2*'°F');
    attribute <'Btu_IT⋅in/(s⋅ft²⋅°F)'> 'British thermal unit (IT) inch per second square foot degree Fahrenheit' : ThermalConductivityUnit = Btu_IT*'in'/(s*ft^2*'°F');
    attribute <'Btu_th⋅in/(s⋅ft²⋅°F)'> 'British thermal unit (th) inch per second square foot degree Fahrenheit' : ThermalConductivityUnit = Btu_th*'in'/(s*ft^2*'°F');
    //attribute <'Btu_IT/ft³'> 'British thermal unit (IT) per cubic foot' : EnergyDensityUnit = Btu_IT/ft^3;
    //attribute <'Btu_th/ft³'> 'British thermal unit (th) per cubic foot' : EnergyDensityUnit = Btu_th/ft^3;
    attribute <'Btu_IT/°F'> 'British thermal unit (IT) per degree Fahrenheit' : HeatCapacityUnit = Btu_IT/'°F';
    attribute <'Btu_th/°F'> 'British thermal unit (th) per degree Fahrenheit' : HeatCapacityUnit = Btu_th/'°F';
    attribute <'Btu_IT/°R'> 'British thermal unit (IT) per degree Rankine' : HeatCapacityUnit = Btu_IT/'°R';
    attribute <'Btu_th/°R'> 'British thermal unit (th) per degree Rankine' : HeatCapacityUnit = Btu_th/'°R';
    attribute <'Btu_IT/h'> 'British thermal unit (IT) per hour' : PowerUnit = Btu_IT/h;
    attribute <'Btu_th/h'> 'British thermal unit (th) per hour' : PowerUnit = Btu_th/h;
    attribute <'Btu_IT/(h⋅ft²⋅°F)'> 'British thermal unit (IT) per hour square foot degree Fahrenheit' : CoefficientOfHeatTransferUnit = Btu_IT/(h*ft^2*'°F');
    attribute <'Btu_th/(h⋅ft²⋅°F)'> 'British thermal unit (th) per hour square foot degree Fahrenheit' : CoefficientOfHeatTransferUnit = Btu_th/(h*ft^2*'°F');
    attribute <'Btu_th/min'> 'British thermal unit (th) per minute' : PowerUnit = Btu_th/min;
    attribute <'Btu_IT/lb'> 'British thermal unit (IT) per pound' : SpecificEnergyUnit = Btu_IT/lb;
    attribute <'Btu_th/lb'> 'British thermal unit (th) per pound' : SpecificEnergyUnit = Btu_th/lb;
    attribute <'Btu_IT/(lb⋅°F)'> 'British thermal unit (IT) per pound degree Fahrenheit' : SpecificHeatCapacityUnit = Btu_IT/(lb*'°F');
    attribute <'Btu_th/(lb⋅°F)'> 'British thermal unit (th) per pound degree Fahrenheit' : SpecificHeatCapacityUnit = Btu_th/(lb*'°F');
    attribute <'Btu_IT/(lb⋅°R)'> 'British thermal unit (IT) per pound degree Rankine' : SpecificHeatCapacityUnit = Btu_IT/(lb*'°R');
    attribute <'Btu_th/(lb⋅°R)'> 'British thermal unit (th) per pound degree Rankine' : SpecificHeatCapacityUnit = Btu_th/(lb*'°R');
    attribute <'Btu_IT/s'> 'British thermal unit (IT) per second' : PowerUnit = Btu_IT/s;
    attribute <'Btu_th/s'> 'British thermal unit (th) per second' : PowerUnit = Btu_th/s;
    attribute <'Btu_IT/(s⋅ft²⋅°F)'> 'British thermal unit (IT) per second square foot degree Fahrenheit' : CoefficientOfHeatTransferUnit = Btu_IT/(s*ft^2*'°F');
    attribute <'Btu_th/(s⋅ft²⋅°F)'> 'British thermal unit (th) per second square foot degree Fahrenheit' : CoefficientOfHeatTransferUnit = Btu_th/(s*ft^2*'°F');
    //attribute <'Btu_IT/ft²'> 'British thermal unit (IT) per square foot' : SurfaceHeatDensityUnit = Btu_IT/ft^2;
    //attribute <'Btu_th/ft²'> 'British thermal unit (th) per square foot' : SurfaceHeatDensityUnit = Btu_th/ft^2;
    attribute <'Btu_IT/(ft²⋅h)'> 'British thermal unit (IT) per square foot hour' : DensityOfHeatFlowRateUnit = Btu_IT/(ft^2*h);
    attribute <'Btu_th/(ft²⋅h)'> 'British thermal unit (th) per square foot hour' : DensityOfHeatFlowRateUnit = Btu_th/(ft^2*h);
    attribute <'Btu_th/(ft²⋅min)'> 'British thermal unit (th) per square foot minute' : DensityOfHeatFlowRateUnit = Btu_th/(ft^2*min);
    attribute <'Btu_IT/(ft²⋅s)'> 'British thermal unit (IT) per square foot second' : DensityOfHeatFlowRateUnit = Btu_IT/(ft^2*s);
    attribute <'Btu_th/(ft²⋅s)'> 'British thermal unit (th) per square foot second' : DensityOfHeatFlowRateUnit = Btu_th/(ft^2*s);
    attribute <'Btu_th/(in²⋅s)'> 'British thermal unit (th) per square inch second' : DensityOfHeatFlowRateUnit = Btu_th/('in'^2*s);
    attribute <bu> 'bushel (US)' : VolumeUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^3; :>> conversionFactor = 3.523907E-02; :>> isExact = false; } }
    attribute <'cd/in²'> 'candela per square inch' : LuminanceUnit = cd/'in'^2;
    attribute <ch> 'chain (based on US survey foot)' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 2.011684E+01; :>> isExact = false; } }
    attribute 'circular mil' : AreaUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^2; :>> conversionFactor = 5.067075E-10; :>> isExact = false; } }
    attribute 'clo' : ThermalInsulanceUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^2*K/W; :>> conversionFactor = 1.55E-01; :>> isExact = false; } }
    attribute 'cord (128 ft^3)' : VolumeUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^3; :>> conversionFactor = 3.624556E+00; :>> isExact = false; } }
    attribute <'ft³'> 'cubic foot' : VolumeUnit = ft^3;
    attribute <'ft³/min'> 'cubic foot per minute' : VolumeFlowRateUnit = ft^3/min;
    attribute <'ft³/s'> 'cubic foot per second' : VolumeFlowRateUnit = ft^3/s;
    attribute <'in³'> 'cubic inch' : VolumeUnit = 'in'^3;
    attribute <'in³/min'> 'cubic inch per minute' : VolumeFlowRateUnit = 'in'^3/min;
    attribute <'mi³'> 'cubic mile' : VolumeUnit = mi^3;
    attribute <'yd³'> 'cubic yard' : VolumeUnit = yd^3;
    attribute <'yd³/min'> 'cubic yard per minute' : VolumeFlowRateUnit = yd^3/min;
    attribute 'cup (US)' : VolumeUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^3; :>> conversionFactor = 2.365882E-04; :>> isExact = false; } }
    attribute <'°F'> 'degree Fahrenheit (temperature difference)' : TemperatureDifferenceUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = K; :>> conversionFactor = 5/9; :>> isExact = true; } }
    attribute <'°F⋅h/Btu_IT'> 'degree Fahrenheit hour per British thermal unit (IT)' : ThermalResistanceUnit = '°F'*h/Btu_IT;
    attribute <'°F⋅h/Btu_th'> 'degree Fahrenheit hour per British thermal unit (th)' : ThermalResistanceUnit = '°F'*h/Btu_th;
    attribute <'°F⋅h⋅ft²/Btu_IT'> 'degree Fahrenheit hour square foot per British thermal unit (IT)' : ThermalInsulanceUnit = '°F'*h*ft^2/Btu_IT;
    attribute <'°F⋅h⋅ft²/Btu_th'> 'degree Fahrenheit hour square foot per British thermal unit (th)' : ThermalInsulanceUnit = '°F'*h*ft^2/Btu_th;
    //attribute <'°F⋅h⋅ft²/(Btu_IT⋅in)'> 'degree Fahrenheit hour square foot per British thermal unit (IT) inch' : ThermalResistivityUnit = '°F'*h*ft^2/(Btu_IT*'in');
    //attribute <'°F⋅h⋅ft²/(Btu_th⋅in)'> 'degree Fahrenheit hour square foot per British thermal unit (th) inch' : ThermalResistivityUnit = '°F'*h*ft^2/(Btu_th*'in');
    attribute <'°F⋅s/Btu_IT'> 'degree Fahrenheit second per British thermal unit (IT)' : ThermalResistanceUnit = '°F'*s/Btu_IT;
    attribute <'°F⋅s/Btu_th'> 'degree Fahrenheit second per British thermal unit (th)' : ThermalResistanceUnit = '°F'*s/Btu_th;
    attribute <'°R'> 'degree Rankine' : ThermodynamicTemperatureUnit, TemperatureDifferenceUnit { 
        :>> unitConversion: ConversionByConvention { :>> referenceUnit = K; :>> conversionFactor = 5/9; :>> isExact = true; }
        :>> ThermodynamicTemperatureUnit::quantityDimension, TemperatureDifferenceUnit::quantityDimension {
            :>> ThermodynamicTemperatureUnit::quantityDimension::quantityPowerFactors, TemperatureDifferenceUnit::quantityDimension::quantityPowerFactors;
        }
    }
    attribute 'fathom (based on US survey foot)' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 1.828804E+00; :>> isExact = false; } }
    attribute <floz> 'fluid ounce (US)' : VolumeUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^3; :>> conversionFactor = 2.957353E-05; :>> isExact = false; } }
    attribute <ft> 'foot' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 3.048E-01; } }
    attribute 'foot (US survey)' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 3.048006E-01; :>> isExact = false; } }
    attribute 'footcandle' : IlluminanceUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = lx; :>> conversionFactor = 1.076391E+01; :>> isExact = false; } }
    attribute 'footlambert' : LuminanceUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = cd/m^2; :>> conversionFactor = 3.426259E+00; :>> isExact = false; } }
    attribute <ftHg> 'foot of mercury, conventional' : PressureUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = Pa; :>> conversionFactor = 4.063666E+04; :>> isExact = false; } }
    attribute 'foot of water (39.2 °F)' : PressureUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = Pa; :>> conversionFactor = 2.98898E+03; :>> isExact = false; } }
    attribute <ftH2O> 'foot of water, conventional' : PressureUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = Pa; :>> conversionFactor = 2.989067E+03; :>> isExact = false; } }
    attribute <'ft/h'> 'foot per hour' : SpeedUnit = ft/h;
    attribute <'ft/min'> 'foot per minute' : SpeedUnit = ft/min;
    attribute <'ft/s'> 'foot per second' : SpeedUnit = ft/s;
    attribute <'ft/s²'> 'foot per second squared' : AccelerationUnit = ft/s^2;
    attribute 'foot poundal' : EnergyUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = J; :>> conversionFactor = 4.214011E-02; :>> isExact = false; } }
    attribute <'ft⋅lbf'> 'foot pound-force' : EnergyUnit = ft*lbf;
    attribute <'ft⋅lbf/h'> 'foot pound-force per hour' : PowerUnit = ft*lbf/h;
    attribute <'ft⋅lbf/min'> 'foot pound-force per minute' : PowerUnit = ft*lbf/min;
    attribute <'ft⋅lbf/s'> 'foot pound-force per second' : PowerUnit = ft*lbf/s;
    attribute <'ft⁴'> 'foot to the fourth power' : SecondAxialMomentOfAreaUnit = ft^4;
    attribute <gal> 'gallon (US)' : VolumeUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^3; :>> conversionFactor = 3.785412E-03; :>> isExact = false; } }
    attribute <'gal/d'> 'gallon (US) per day' : VolumeFlowRateUnit = gal/d;
    //attribute <'gal/(hp⋅h)'> 'gallon (US) per horsepower hour' : EnergySpecificVolumeUnit = gal/(hp*h);
    attribute <'gal/min'> 'gallon (US) per minute (gpm)' : VolumeFlowRateUnit = gal/min;
    attribute <gi> 'gill (US)' : VolumeUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^3; :>> conversionFactor = 1.182941E-04; :>> isExact = false; } }
    attribute <gr> 'grain' : MassUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg; :>> conversionFactor = 6.479891E-05; } }
    attribute <'gr/gal'> 'grain per gallon (US)' : MassDensityUnit = gr/gal;
    attribute <hp> 'horsepower (550 ft*lbf/s)' : PowerUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = W; :>> conversionFactor = 7.456999E+02; :>> isExact = false; } }
    attribute 'horsepower (boiler)' : PowerUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = W; :>> conversionFactor = 9.80950E+03; :>> isExact = false; } }
    attribute 'horsepower (electric)' : PowerUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = W; :>> conversionFactor = 7.46E+02; } }
    attribute 'horsepower (water)' : PowerUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = W; :>> conversionFactor = 7.46043E+02; :>> isExact = false; } }
    attribute 'hundredweight (long, 112 lb)' : MassUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg; :>> conversionFactor = 5.080235E+01; :>> isExact = false; } }
    attribute 'hundredweight (short, 100 lb)' : MassUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg; :>> conversionFactor = 4.535924E+01; :>> isExact = false; } }
    attribute <'in'> 'inch' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 2.54E-02; } }
    attribute 'inch of mercury (32 °F)' : PressureUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = Pa; :>> conversionFactor = 3.38638E+03; :>> isExact = false; } }
    attribute 'inch of mercury (60 °F)' : PressureUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = Pa; :>> conversionFactor = 3.37685E+03; :>> isExact = false; } }
    attribute <inHg> 'inch of mercury, conventional' : PressureUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = Pa; :>> conversionFactor = 3.386389E+03; :>> isExact = false; } }
    attribute 'inch of water (39.2 °F)' : PressureUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = Pa; :>> conversionFactor = 2.49082E+02; :>> isExact = false; } }
    attribute 'inch of water (60 °F)' : PressureUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = Pa; :>> conversionFactor = 2.4884E+02; :>> isExact = false; } }
    attribute <inH2O> 'inch of water, conventional' : PressureUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = Pa; :>> conversionFactor = 2.490889E+02; :>> isExact = false; } }
    attribute <'in/s'> 'inch per second' : SpeedUnit = 'in'/s;
    attribute <'in/s²'> 'inch per second squared' : AccelerationUnit = 'in'/s^2;
    attribute <'in⁴'> 'inch to the fourth power' : SecondAxialMomentOfAreaUnit = 'in'^4;
    attribute <kip> 'kip (1 kip = 1000 lbf)' : ForceUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = N; :>> conversionFactor = 4.448222E+03; :>> isExact = false; } }
    attribute <'kip/in²'> 'kip per square inch (ksi)' : PressureUnit = kip/'in'^2;
    attribute <knot> 'knot (nautical mile per hour)' : SpeedUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m/s; :>> conversionFactor = 5.144444E-01; :>> isExact = false; } }
    //attribute <'cal_th/cm²'> 'langley' : SurfaceHeatDensityUnit = cal_th/cm^2;
    attribute <'lm/ft²'> 'lumen per square foot' : IlluminanceUnit = lm/ft^2;
    attribute 'microinch' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 2.54E-08; } }
    attribute <mil> 'mil (0.001 in)' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 2.54E-05; } }
    attribute <mi> 'mile' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 1.609344E+03; } }
    attribute 'mile (based on US survey foot)' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 1.609347E+03; :>> isExact = false; } }
    attribute <nmi> 'mile, nautical' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 1.852E+03; } }
    alias NM for nmi;
    //attribute <'mi/gal'> 'mile per gallon (US)' : FuelEconomyUnit = mi/gal;
    //alias mpg for 'mi/gal';
    attribute <'mi/h'> 'mile per hour' : SpeedUnit = mi/h;
    alias mph for 'mi/h';
    attribute <'mi/min'> 'mile per minute' : SpeedUnit = mi/min;
    attribute <'mi/s'> 'mile per second' : SpeedUnit = mi/s;
    attribute 'ohm circular-mil per foot' : ResistivityUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = 'Ω'*m; :>> conversionFactor = 1.662426E-09; :>> isExact = false; } }
    attribute <oz> 'ounce (avoirdupois)' : MassUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg; :>> conversionFactor = 2.834952E-02; :>> isExact = false; } }
    attribute 'ounce (US fluid)' : VolumeUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^3; :>> conversionFactor = 2.957353E-05; :>> isExact = false; } }
    attribute <ozf> 'ounce (avoirdupois)-force' : ForceUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = N; :>> conversionFactor = 2.780139E-01; :>> isExact = false; } }
    attribute <'ozf⋅in'> 'ounce (avoirdupois)-force inch' : MomentOfForceUnit = ozf*'in';
    attribute <'oz/in³'> 'ounce (avoirdupois) per cubic inch' : MassDensityUnit = oz/'in'^3;
    attribute <'oz/gal'> 'ounce (avoirdupois) per gallon (US)' : MassDensityUnit = oz/gal;
    attribute <'oz/ft²'> 'ounce (avoirdupois) per square foot' : SurfaceMassDensityUnit = oz/ft^2;
    attribute <'oz/in²'> 'ounce (avoirdupois) per square inch' : SurfaceMassDensityUnit = oz/'in'^2;
    attribute <'oz/yd²'> 'ounce (avoirdupois) per square yard' : SurfaceMassDensityUnit = oz/yd^2;
    attribute <pk> 'peck (US)' : VolumeUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^3; :>> conversionFactor = 8.809768E-03; :>> isExact = false; } }
    //attribute 'perm (0 °C)' : VapourTransmissionUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg/(Pa*s*m^2); :>> conversionFactor = 5.72135E-11; :>> isExact = false; } }
    //attribute 'perm (23 °C)' : VapourTransmissionUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg/(Pa*s*m^2); :>> conversionFactor = 5.74525E-11; :>> isExact = false; } }
    //attribute 'perm inch (0 °C)' : VapourTransmissionUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg/('Pa·s·m'); :>> conversionFactor = 1.45322E-12; :>> isExact = false; } }
    //attribute 'perm inch (23 °C)' : VapourTransmissionUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg/('Pa·s·m'); :>> conversionFactor = 1.45929E-12; :>> isExact = false; } }
    attribute <pica> 'pica (computer) (1/6 in)' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 4.233333E-03; :>> isExact = false; } }
    attribute 'pica (printer′s)' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 4.217518E-03; :>> isExact = false; } }
    attribute <drypt> 'pint (US dry)' : VolumeUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^3; :>> conversionFactor = 5.506105E-04; :>> isExact = false; } }
    attribute <liqpt> 'pint (US liquid)' : VolumeUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^3; :>> conversionFactor = 4.731765E-04; :>> isExact = false; } }
    attribute <pt> 'point (computer) (1/72 in)' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 3.527778E-04; :>> isExact = false; } }
    attribute 'point (printer′s)' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 3.514598E-04; :>> isExact = false; } }
    attribute <lb> 'pound (avoirdupois)' : MassUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg; :>> conversionFactor = 4.535924E-01; :>> isExact = false; } }
    attribute <'lb⋅ft²'> 'pound foot squared' : MomentOfInertiaUnit = lb*ft^2;
    attribute <lbf> 'pound-force' : ForceUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = N; :>> conversionFactor = 4.448222E+00; :>> isExact = false; } }
    attribute <'lbf⋅ft'> 'pound-force foot' : MomentOfForceUnit = lbf*ft;
    attribute <'lbf⋅ft/in'> 'pound-force foot per inch' : ForceUnit = lbf*ft/'in';
    attribute <'lbf⋅in'> 'pound-force inch' : MomentOfForceUnit = lbf*'in';
    attribute <'lbf⋅in/in'> 'pound-force inch per inch' : ForceUnit = lbf*'in'/'in';
    attribute <'lbf/ft'> 'pound-force per foot' : SurfaceTensionUnit = lbf/ft;
    attribute <'lbf/in'> 'pound-force per inch' : SurfaceTensionUnit = lbf/'in';
    //attribute 'pound-force per pound (lbf/lb) (thrust to mass ratio)' : ThrustToMassRatioUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = N/kg; :>> conversionFactor = 9.80665E+00; } }
    attribute <'lbf/ft²'> 'pound-force per square foot' : PressureUnit = lbf/ft^2;
    attribute <'lbf/in²'> 'pound-force per square inch' : PressureUnit = lbf/'in'^2;
    alias psi for 'lbf/in²';
    attribute <'lbf⋅s/ft²'> 'pound-force second per square foot' : DynamicViscosityUnit = lbf*s/ft^2;
    attribute <'lbf⋅s/in²'> 'pound-force second per square inch' : DynamicViscosityUnit = lbf*s/'in'^2;
    attribute <'lb⋅in²'> 'pound inch squared' : MomentOfInertiaUnit = lb*'in'^2;
    attribute <'lb/ft³'> 'pound per cubic foot' : MassDensityUnit = lb/ft^3;
    attribute <'lb/in³'> 'pound per cubic inch' : MassDensityUnit = lb/'in'^3;
    attribute <'lb/yd³'> 'pound per cubic yard' : MassDensityUnit = lb/yd^3;
    attribute <'lb/ft'> 'pound per foot' : LinearMassDensityUnit = lb/ft;
    attribute <'lb/(ft⋅h)'> 'pound per foot hour' : DynamicViscosityUnit = lb/(ft*h);
    attribute <'lb/(ft⋅s)'> 'pound per foot second' : DynamicViscosityUnit = lb/(ft*s);
    attribute <'lb/gal'> 'pound per gallon (US)' : MassDensityUnit = lb/gal;
    //attribute <'lb/(hp⋅h)'> 'pound per horsepower hour' : FuelConsumptionUnit = lb/(hp*h);
    attribute <'lb/h'> 'pound per hour' : MassFlowRateUnit = lb/h;
    attribute <'lb/in'> 'pound per inch' : LinearMassDensityUnit = lb/'in';
    attribute <'lb/min'> 'pound per minute' : MassFlowRateUnit = lb/min;
    attribute <'lb/s'> 'pound per second' : MassFlowRateUnit = lb/s;
    attribute <'lb/ft²'> 'pound per square foot' : SurfaceMassDensityUnit = lb/ft^2;
    attribute <'lb/in²'> 'pound per square inch (not pound-force)' : SurfaceMassDensityUnit = lb/'in'^2;
    attribute <'lb/yd'> 'pound per yard' : LinearMassDensityUnit = lb/yd;
    attribute 'pound-force per square inch (psi)' : PressureUnit = lbf/'in'^2;
    attribute 'quad (10^15 Btu_IT)' : EnergyUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = J; :>> conversionFactor = 1.055056E+18; :>> isExact = false; } }
    attribute <dryqt> 'quart (US dry)' : VolumeUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^3; :>> conversionFactor = 1.101221E-03; :>> isExact = false; } }
    attribute <liqqt> 'quart (US liquid)' : VolumeUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^3; :>> conversionFactor = 9.463529E-04; :>> isExact = false; } }
    attribute <rd> 'rod (based on US survey foot)' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 5.029210E+00; :>> isExact = false; } }
    attribute <slug> 'slug' : MassUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg; :>> conversionFactor = 1.459390E+01; :>> isExact = false; } }
    attribute <'slug/ft³'> 'slug per cubic foot' : MassDensityUnit = slug/ft^3;
    attribute <'slug/(ft⋅s)'> 'slug per foot second' : DynamicViscosityUnit = slug/(ft*s);
    attribute <'ft²'> 'square foot' : AreaUnit = ft^2;
    attribute <'ft²/h'> 'square foot per hour' : KinematicViscosityUnit = ft^2/h;
    attribute <'ft²/s'> 'square foot per second' : KinematicViscosityUnit = ft^2/s;
    attribute <'in²'> 'square inch' : AreaUnit = 'in'^2;
    attribute <'mi²'> 'square mile' : AreaUnit = mi^2;
    attribute 'square mile (based on US survey foot)' : AreaUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^2; :>> conversionFactor = 2.589998E+06; :>> isExact = false; } }
    attribute <'yd²'> 'square yard' : AreaUnit = yd^2;
    attribute 'tablespoon' : VolumeUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^3; :>> conversionFactor = 1.478676E-05; :>> isExact = false; } }
    attribute 'teaspoon' : VolumeUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^3; :>> conversionFactor = 4.928922E-06; :>> isExact = false; } }
    attribute 'therm (EC)' : EnergyUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = J; :>> conversionFactor = 1.05506E+08; } }
    attribute 'therm (US)' : EnergyUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = J; :>> conversionFactor = 1.054804E+08; } }
    attribute <AT> 'ton, assay' : MassUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg; :>> conversionFactor = 2.916667E-02; :>> isExact = false; } }
    attribute 'ton-force (2000 lbf)' : ForceUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = N; :>> conversionFactor = 8.896443E+03; :>> isExact = false; } }
    attribute 'ton, long (2240 lb)' : MassUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg; :>> conversionFactor = 1.016047E+03; :>> isExact = false; } }
    attribute 'ton, long, per cubic yard' : MassDensityUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg/m^3; :>> conversionFactor = 1.328939E+03; :>> isExact = false; } }
    attribute 'ton of refrigeration (12 000 Btu_IT/h)' : PowerUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = W; :>> conversionFactor = 3.516853E+03; :>> isExact = false; } }
    attribute 'ton, register' : VolumeUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^3; :>> conversionFactor = 2.831685E+00; :>> isExact = false; } }
    attribute 'ton, short (2000 lb)' : MassUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg; :>> conversionFactor = 9.071847E+02; :>> isExact = false; } }
    attribute 'ton, short, per cubic yard' : MassDensityUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg/m^3; :>> conversionFactor = 1.186553E+03; :>> isExact = false; } }
    attribute 'ton, short, per hour' : MassFlowRateUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg/s; :>> conversionFactor = 2.519958E-01; :>> isExact = false; } }
    attribute 'unit pole' : MagneticFluxUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = Wb; :>> conversionFactor = 1.256637E-07; :>> isExact = false; } }
    attribute <'W/in²'> 'watt per square inch' : DensityOfHeatFlowRateUnit = W/'in'^2;
    attribute <yd> 'yard' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 9.144E-01; } }


	attribute <'°F_abs'> 'degree fahrenheit (absolute temperature scale)' : IntervalScale {
		doc
		/*
		 * degree Fahrenheit interval scale for absolute (thermodynamic) temperature quantities
		 *
	     * The interval scale is defined with an explicit transformation with respect to 
	     * the kelvin thermodynamic temperature scale that specifies the zero shift.
		 */
		
		:>> unit = '°F';
		private attribute temperatureWaterAtFreezingPointInF: DefinitionalQuantityValue {
			:>> num = 32.0;
			:>> definition = "temperature in degree Fahrenheit of pure water at freezing point";
		}
		private attribute fahrenheitToCelsiusScaleMapping: QuantityValueMapping {
			:>> mappedQuantityValue = temperatureWaterAtFreezingPointInF;
			:>> referenceQuantityValue = '°C_abs'.temperatureWaterAtFreezingPointInC;

		}
		attribute :>> definitionalQuantityValues = temperatureWaterAtFreezingPointInF;
		attribute :>> quantityValueMapping = fahrenheitToCelsiusScaleMapping;

        /* CoordinateFramePlacement (zero shift) w.r.t. the kelvin thermodynamic temperature scale */
        private attribute zeroDegreeFahrenheitInKelvin: ThermodynamicTemperatureValue = 229835/900 [K];
        attribute zeroDegreeFahrenheitToKelvinShift : CoordinateFramePlacement :>> transformation { 
        	:>> source = K; :>> origin = zeroDegreeFahrenheitInKelvin;
        }
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'AreaUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
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
semantic.unresolved_name 'EnergyUnit'
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
semantic.unresolved_name 'EnergyUnit'
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
semantic.unresolved_name 'EnergyUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'ThermalConductivityUnit'
semantic.unresolved_name 'ThermalConductivityUnit'
semantic.unresolved_name 'ThermalConductivityUnit'
semantic.unresolved_name 'ThermalConductivityUnit'
semantic.unresolved_name 'ThermalConductivityUnit'
semantic.unresolved_name 'ThermalConductivityUnit'
semantic.unresolved_name 'HeatCapacityUnit'
semantic.unresolved_name 'HeatCapacityUnit'
semantic.unresolved_name 'HeatCapacityUnit'
semantic.unresolved_name 'HeatCapacityUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'CoefficientOfHeatTransferUnit'
semantic.unresolved_name 'CoefficientOfHeatTransferUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'SpecificEnergyUnit'
semantic.unresolved_name 'SpecificEnergyUnit'
semantic.unresolved_name 'SpecificHeatCapacityUnit'
semantic.unresolved_name 'SpecificHeatCapacityUnit'
semantic.unresolved_name 'SpecificHeatCapacityUnit'
semantic.unresolved_name 'SpecificHeatCapacityUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'CoefficientOfHeatTransferUnit'
semantic.unresolved_name 'CoefficientOfHeatTransferUnit'
semantic.unresolved_name 'DensityOfHeatFlowRateUnit'
semantic.unresolved_name 'DensityOfHeatFlowRateUnit'
semantic.unresolved_name 'DensityOfHeatFlowRateUnit'
semantic.unresolved_name 'DensityOfHeatFlowRateUnit'
semantic.unresolved_name 'DensityOfHeatFlowRateUnit'
semantic.unresolved_name 'DensityOfHeatFlowRateUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'LuminanceUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'AreaUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'ThermalInsulanceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'VolumeFlowRateUnit'
semantic.unresolved_name 'VolumeFlowRateUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'VolumeFlowRateUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'VolumeFlowRateUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'TemperatureDifferenceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'ThermalResistanceUnit'
semantic.unresolved_name 'ThermalResistanceUnit'
semantic.unresolved_name 'ThermalInsulanceUnit'
semantic.unresolved_name 'ThermalInsulanceUnit'
semantic.unresolved_name 'ThermalResistanceUnit'
semantic.unresolved_name 'ThermalResistanceUnit'
semantic.unresolved_name 'ThermodynamicTemperatureUnit'
semantic.unresolved_name 'TemperatureDifferenceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'ThermodynamicTemperatureUnit::quantityDimension'
semantic.unresolved_name 'TemperatureDifferenceUnit::quantityDimension'
semantic.unresolved_name 'ThermodynamicTemperatureUnit::quantityDimension::quantityPowerFactors'
semantic.unresolved_name 'TemperatureDifferenceUnit::quantityDimension::quantityPowerFactors'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'LengthUnit'
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
semantic.unresolved_name 'IlluminanceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'LuminanceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'AccelerationUnit'
semantic.unresolved_name 'EnergyUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'EnergyUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'SecondAxialMomentOfAreaUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeFlowRateUnit'
semantic.unresolved_name 'VolumeFlowRateUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'MassDensityUnit'
semantic.unresolved_name 'PowerUnit'
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
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'AccelerationUnit'
semantic.unresolved_name 'SecondAxialMomentOfAreaUnit'
semantic.unresolved_name 'ForceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'IlluminanceUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'LengthUnit'
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
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'ResistivityUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'ForceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MomentOfForceUnit'
semantic.unresolved_name 'MassDensityUnit'
semantic.unresolved_name 'MassDensityUnit'
semantic.unresolved_name 'SurfaceMassDensityUnit'
semantic.unresolved_name 'SurfaceMassDensityUnit'
semantic.unresolved_name 'SurfaceMassDensityUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MomentOfInertiaUnit'
semantic.unresolved_name 'ForceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MomentOfForceUnit'
semantic.unresolved_name 'ForceUnit'
semantic.unresolved_name 'MomentOfForceUnit'
semantic.unresolved_name 'ForceUnit'
semantic.unresolved_name 'SurfaceTensionUnit'
semantic.unresolved_name 'SurfaceTensionUnit'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'DynamicViscosityUnit'
semantic.unresolved_name 'DynamicViscosityUnit'
semantic.unresolved_name 'MomentOfInertiaUnit'
semantic.unresolved_name 'MassDensityUnit'
semantic.unresolved_name 'MassDensityUnit'
semantic.unresolved_name 'MassDensityUnit'
semantic.unresolved_name 'LinearMassDensityUnit'
semantic.unresolved_name 'DynamicViscosityUnit'
semantic.unresolved_name 'DynamicViscosityUnit'
semantic.unresolved_name 'MassDensityUnit'
semantic.unresolved_name 'MassFlowRateUnit'
semantic.unresolved_name 'LinearMassDensityUnit'
semantic.unresolved_name 'MassFlowRateUnit'
semantic.unresolved_name 'MassFlowRateUnit'
semantic.unresolved_name 'SurfaceMassDensityUnit'
semantic.unresolved_name 'SurfaceMassDensityUnit'
semantic.unresolved_name 'LinearMassDensityUnit'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'EnergyUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassDensityUnit'
semantic.unresolved_name 'DynamicViscosityUnit'
semantic.unresolved_name 'AreaUnit'
semantic.unresolved_name 'KinematicViscosityUnit'
semantic.unresolved_name 'KinematicViscosityUnit'
semantic.unresolved_name 'AreaUnit'
semantic.unresolved_name 'AreaUnit'
semantic.unresolved_name 'AreaUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'AreaUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
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
semantic.unresolved_name 'EnergyUnit'
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
semantic.unresolved_name 'ForceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassDensityUnit'
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
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassDensityUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassFlowRateUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MagneticFluxUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'DensityOfHeatFlowRateUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'IntervalScale'
semantic.unresolved_name 'unit'
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
semantic.unresolved_name 'AreaUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
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
semantic.unresolved_name 'EnergyUnit'
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
semantic.unresolved_name 'EnergyUnit'
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
semantic.unresolved_name 'EnergyUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'ThermalConductivityUnit'
semantic.unresolved_name 'ThermalConductivityUnit'
semantic.unresolved_name 'ThermalConductivityUnit'
semantic.unresolved_name 'ThermalConductivityUnit'
semantic.unresolved_name 'ThermalConductivityUnit'
semantic.unresolved_name 'ThermalConductivityUnit'
semantic.unresolved_name 'HeatCapacityUnit'
semantic.unresolved_name 'HeatCapacityUnit'
semantic.unresolved_name 'HeatCapacityUnit'
semantic.unresolved_name 'HeatCapacityUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'CoefficientOfHeatTransferUnit'
semantic.unresolved_name 'CoefficientOfHeatTransferUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'SpecificEnergyUnit'
semantic.unresolved_name 'SpecificEnergyUnit'
semantic.unresolved_name 'SpecificHeatCapacityUnit'
semantic.unresolved_name 'SpecificHeatCapacityUnit'
semantic.unresolved_name 'SpecificHeatCapacityUnit'
semantic.unresolved_name 'SpecificHeatCapacityUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'CoefficientOfHeatTransferUnit'
semantic.unresolved_name 'CoefficientOfHeatTransferUnit'
semantic.unresolved_name 'DensityOfHeatFlowRateUnit'
semantic.unresolved_name 'DensityOfHeatFlowRateUnit'
semantic.unresolved_name 'DensityOfHeatFlowRateUnit'
semantic.unresolved_name 'DensityOfHeatFlowRateUnit'
semantic.unresolved_name 'DensityOfHeatFlowRateUnit'
semantic.unresolved_name 'DensityOfHeatFlowRateUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'LuminanceUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'AreaUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'ThermalInsulanceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'VolumeFlowRateUnit'
semantic.unresolved_name 'VolumeFlowRateUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'VolumeFlowRateUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'VolumeFlowRateUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'TemperatureDifferenceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'ThermalResistanceUnit'
semantic.unresolved_name 'ThermalResistanceUnit'
semantic.unresolved_name 'ThermalInsulanceUnit'
semantic.unresolved_name 'ThermalInsulanceUnit'
semantic.unresolved_name 'ThermalResistanceUnit'
semantic.unresolved_name 'ThermalResistanceUnit'
semantic.unresolved_name 'ThermodynamicTemperatureUnit'
semantic.unresolved_name 'TemperatureDifferenceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'ThermodynamicTemperatureUnit::quantityDimension'
semantic.unresolved_name 'TemperatureDifferenceUnit::quantityDimension'
semantic.unresolved_name 'ThermodynamicTemperatureUnit::quantityDimension::quantityPowerFactors'
semantic.unresolved_name 'TemperatureDifferenceUnit::quantityDimension::quantityPowerFactors'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'LengthUnit'
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
semantic.unresolved_name 'IlluminanceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'LuminanceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'AccelerationUnit'
semantic.unresolved_name 'EnergyUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'EnergyUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'SecondAxialMomentOfAreaUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeFlowRateUnit'
semantic.unresolved_name 'VolumeFlowRateUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'MassDensityUnit'
semantic.unresolved_name 'PowerUnit'
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
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'AccelerationUnit'
semantic.unresolved_name 'SecondAxialMomentOfAreaUnit'
semantic.unresolved_name 'ForceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'IlluminanceUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'LengthUnit'
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
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'ResistivityUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'ForceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MomentOfForceUnit'
semantic.unresolved_name 'MassDensityUnit'
semantic.unresolved_name 'MassDensityUnit'
semantic.unresolved_name 'SurfaceMassDensityUnit'
semantic.unresolved_name 'SurfaceMassDensityUnit'
semantic.unresolved_name 'SurfaceMassDensityUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MomentOfInertiaUnit'
semantic.unresolved_name 'ForceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MomentOfForceUnit'
semantic.unresolved_name 'ForceUnit'
semantic.unresolved_name 'MomentOfForceUnit'
semantic.unresolved_name 'ForceUnit'
semantic.unresolved_name 'SurfaceTensionUnit'
semantic.unresolved_name 'SurfaceTensionUnit'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'DynamicViscosityUnit'
semantic.unresolved_name 'DynamicViscosityUnit'
semantic.unresolved_name 'MomentOfInertiaUnit'
semantic.unresolved_name 'MassDensityUnit'
semantic.unresolved_name 'MassDensityUnit'
semantic.unresolved_name 'MassDensityUnit'
semantic.unresolved_name 'LinearMassDensityUnit'
semantic.unresolved_name 'DynamicViscosityUnit'
semantic.unresolved_name 'DynamicViscosityUnit'
semantic.unresolved_name 'MassDensityUnit'
semantic.unresolved_name 'MassFlowRateUnit'
semantic.unresolved_name 'LinearMassDensityUnit'
semantic.unresolved_name 'MassFlowRateUnit'
semantic.unresolved_name 'MassFlowRateUnit'
semantic.unresolved_name 'SurfaceMassDensityUnit'
semantic.unresolved_name 'SurfaceMassDensityUnit'
semantic.unresolved_name 'LinearMassDensityUnit'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'EnergyUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassDensityUnit'
semantic.unresolved_name 'DynamicViscosityUnit'
semantic.unresolved_name 'AreaUnit'
semantic.unresolved_name 'KinematicViscosityUnit'
semantic.unresolved_name 'KinematicViscosityUnit'
semantic.unresolved_name 'AreaUnit'
semantic.unresolved_name 'AreaUnit'
semantic.unresolved_name 'AreaUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'AreaUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
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
semantic.unresolved_name 'EnergyUnit'
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
semantic.unresolved_name 'ForceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassDensityUnit'
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
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassDensityUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MassFlowRateUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'MagneticFluxUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'DensityOfHeatFlowRateUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'IntervalScale'
semantic.unresolved_name 'unit'
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
KwStandard,KwLibrary,KwPackage,OpenAngle,Ident,CloseAngle,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Slash,OpenParen,Ident,Star,Ident,Caret,DecimalValue,Star,UnrestrictedName,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Slash,OpenParen,Ident,Star,Ident,Caret,DecimalValue,Star,UnrestrictedName,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,UnrestrictedName,Slash,OpenParen,Ident,Star,Ident,Caret,DecimalValue,Star,UnrestrictedName,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,UnrestrictedName,Slash,OpenParen,Ident,Star,Ident,Caret,DecimalValue,Star,UnrestrictedName,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,UnrestrictedName,Slash,OpenParen,Ident,Star,Ident,Caret,DecimalValue,Star,UnrestrictedName,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,UnrestrictedName,Slash,OpenParen,Ident,Star,Ident,Caret,DecimalValue,Star,UnrestrictedName,CloseParen,Semicolon,
LineComment,
LineComment,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,UnrestrictedName,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,UnrestrictedName,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,UnrestrictedName,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,UnrestrictedName,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,Ident,Caret,DecimalValue,Star,UnrestrictedName,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,Ident,Caret,DecimalValue,Star,UnrestrictedName,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,UnrestrictedName,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,UnrestrictedName,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,UnrestrictedName,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,UnrestrictedName,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,Ident,Caret,DecimalValue,Star,UnrestrictedName,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,Ident,Caret,DecimalValue,Star,UnrestrictedName,CloseParen,Semicolon,
LineComment,
LineComment,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Caret,DecimalValue,Star,Ident,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Caret,DecimalValue,Star,Ident,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Caret,DecimalValue,Star,Ident,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Caret,DecimalValue,Star,Ident,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Caret,DecimalValue,Star,Ident,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,UnrestrictedName,Caret,DecimalValue,Star,Ident,CloseParen,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,UnrestrictedName,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Caret,DecimalValue,Star,Ident,Slash,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,UnrestrictedName,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,UnrestrictedName,Caret,DecimalValue,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Slash,Ident,Semicolon,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Slash,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,KwTrue,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,UnrestrictedName,Star,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,UnrestrictedName,Star,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,UnrestrictedName,Star,Ident,Star,Ident,Caret,DecimalValue,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,UnrestrictedName,Star,Ident,Star,Ident,Caret,DecimalValue,Slash,Ident,Semicolon,
LineComment,
LineComment,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,UnrestrictedName,Star,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,UnrestrictedName,Star,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Comma,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Slash,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,KwTrue,Semicolon,CloseCurly,
ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,OpenCurly,
ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
LineComment,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,UnrestrictedName,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,UnrestrictedName,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,UnrestrictedName,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,UnrestrictedName,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Slash,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
LineComment,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,CloseCurly,CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
LineComment,
LineComment,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAlias,Ident,KwFor,UnrestrictedName,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,UnrestrictedName,Star,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,UnrestrictedName,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,UnrestrictedName,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,UnrestrictedName,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
LineComment,
LineComment,
LineComment,
LineComment,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Slash,UnrestrictedName,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,UnrestrictedName,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,UnrestrictedName,Slash,UnrestrictedName,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,UnrestrictedName,Semicolon,
LineComment,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,UnrestrictedName,Caret,DecimalValue,Semicolon,
KwAlias,Ident,KwFor,UnrestrictedName,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Slash,UnrestrictedName,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,UnrestrictedName,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,UnrestrictedName,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,Ident,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,Ident,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
LineComment,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,UnrestrictedName,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,UnrestrictedName,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,UnrestrictedName,Caret,DecimalValue,Semicolon,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,Ident,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,UnrestrictedName,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Slash,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,UnrestrictedName,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
ColonGtGt,Ident,Eq,UnrestrictedName,Semicolon,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
ColonGtGt,Ident,Eq,StringValue,Semicolon,
CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,UnrestrictedName,Dot,Ident,Semicolon,
CloseCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,
RegularComment,
KwPrivate,KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,Slash,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'USCustomaryUnits'
    (documentation)
    (import_decl private 'MeasurementReferences::*')
    (import_decl public 'ISQ::*')
    (import_decl private 'SI::*')
    (attribute_usage ''acre (based on US survey foot)'' : 'AreaUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''acre foot (based on US survey foot)'' : 'VolumeUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''barrel (for petroleum, 42 gallons (US))'' : 'VolumeUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''British thermal unit (IT)'' : 'EnergyUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (alias_member 'Btu' for 'Btu_IT')
    (attribute_usage ''British thermal unit (th)'' : 'EnergyUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''British thermal unit (mean)'' : 'EnergyUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''British thermal unit (39 °F)'' : 'EnergyUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''British thermal unit (59 °F)'' : 'EnergyUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''British thermal unit (60 °F)'' : 'EnergyUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''British thermal unit (IT) foot per hour square foot degree Fahrenheit'' : 'ThermalConductivityUnit' value)
    (attribute_usage ''British thermal unit (th) foot per hour square foot degree Fahrenheit'' : 'ThermalConductivityUnit' value)
    (attribute_usage ''British thermal unit (IT) inch per hour square foot degree Fahrenheit'' : 'ThermalConductivityUnit' value)
    (attribute_usage ''British thermal unit (th) inch per hour square foot degree Fahrenheit'' : 'ThermalConductivityUnit' value)
    (attribute_usage ''British thermal unit (IT) inch per second square foot degree Fahrenheit'' : 'ThermalConductivityUnit' value)
    (attribute_usage ''British thermal unit (th) inch per second square foot degree Fahrenheit'' : 'ThermalConductivityUnit' value)
    (line_comment)
    (line_comment)
    (attribute_usage ''British thermal unit (IT) per degree Fahrenheit'' : 'HeatCapacityUnit' value)
    (attribute_usage ''British thermal unit (th) per degree Fahrenheit'' : 'HeatCapacityUnit' value)
    (attribute_usage ''British thermal unit (IT) per degree Rankine'' : 'HeatCapacityUnit' value)
    (attribute_usage ''British thermal unit (th) per degree Rankine'' : 'HeatCapacityUnit' value)
    (attribute_usage ''British thermal unit (IT) per hour'' : 'PowerUnit' value)
    (attribute_usage ''British thermal unit (th) per hour'' : 'PowerUnit' value)
    (attribute_usage ''British thermal unit (IT) per hour square foot degree Fahrenheit'' : 'CoefficientOfHeatTransferUnit' value)
    (attribute_usage ''British thermal unit (th) per hour square foot degree Fahrenheit'' : 'CoefficientOfHeatTransferUnit' value)
    (attribute_usage ''British thermal unit (th) per minute'' : 'PowerUnit' value)
    (attribute_usage ''British thermal unit (IT) per pound'' : 'SpecificEnergyUnit' value)
    (attribute_usage ''British thermal unit (th) per pound'' : 'SpecificEnergyUnit' value)
    (attribute_usage ''British thermal unit (IT) per pound degree Fahrenheit'' : 'SpecificHeatCapacityUnit' value)
    (attribute_usage ''British thermal unit (th) per pound degree Fahrenheit'' : 'SpecificHeatCapacityUnit' value)
    (attribute_usage ''British thermal unit (IT) per pound degree Rankine'' : 'SpecificHeatCapacityUnit' value)
    (attribute_usage ''British thermal unit (th) per pound degree Rankine'' : 'SpecificHeatCapacityUnit' value)
    (attribute_usage ''British thermal unit (IT) per second'' : 'PowerUnit' value)
    (attribute_usage ''British thermal unit (th) per second'' : 'PowerUnit' value)
    (attribute_usage ''British thermal unit (IT) per second square foot degree Fahrenheit'' : 'CoefficientOfHeatTransferUnit' value)
    (attribute_usage ''British thermal unit (th) per second square foot degree Fahrenheit'' : 'CoefficientOfHeatTransferUnit' value)
    (line_comment)
    (line_comment)
    (attribute_usage ''British thermal unit (IT) per square foot hour'' : 'DensityOfHeatFlowRateUnit' value)
    (attribute_usage ''British thermal unit (th) per square foot hour'' : 'DensityOfHeatFlowRateUnit' value)
    (attribute_usage ''British thermal unit (th) per square foot minute'' : 'DensityOfHeatFlowRateUnit' value)
    (attribute_usage ''British thermal unit (IT) per square foot second'' : 'DensityOfHeatFlowRateUnit' value)
    (attribute_usage ''British thermal unit (th) per square foot second'' : 'DensityOfHeatFlowRateUnit' value)
    (attribute_usage ''British thermal unit (th) per square inch second'' : 'DensityOfHeatFlowRateUnit' value)
    (attribute_usage ''bushel (US)'' : 'VolumeUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''candela per square inch'' : 'LuminanceUnit' value)
    (attribute_usage ''chain (based on US survey foot)'' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''circular mil'' : 'AreaUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''clo'' : 'ThermalInsulanceUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''cord (128 ft^3)'' : 'VolumeUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''cubic foot'' : 'VolumeUnit' value)
    (attribute_usage ''cubic foot per minute'' : 'VolumeFlowRateUnit' value)
    (attribute_usage ''cubic foot per second'' : 'VolumeFlowRateUnit' value)
    (attribute_usage ''cubic inch'' : 'VolumeUnit' value)
    (attribute_usage ''cubic inch per minute'' : 'VolumeFlowRateUnit' value)
    (attribute_usage ''cubic mile'' : 'VolumeUnit' value)
    (attribute_usage ''cubic yard'' : 'VolumeUnit' value)
    (attribute_usage ''cubic yard per minute'' : 'VolumeFlowRateUnit' value)
    (attribute_usage ''cup (US)'' : 'VolumeUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''degree Fahrenheit (temperature difference)'' : 'TemperatureDifferenceUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''degree Fahrenheit hour per British thermal unit (IT)'' : 'ThermalResistanceUnit' value)
    (attribute_usage ''degree Fahrenheit hour per British thermal unit (th)'' : 'ThermalResistanceUnit' value)
    (attribute_usage ''degree Fahrenheit hour square foot per British thermal unit (IT)'' : 'ThermalInsulanceUnit' value)
    (attribute_usage ''degree Fahrenheit hour square foot per British thermal unit (th)'' : 'ThermalInsulanceUnit' value)
    (line_comment)
    (line_comment)
    (attribute_usage ''degree Fahrenheit second per British thermal unit (IT)'' : 'ThermalResistanceUnit' value)
    (attribute_usage ''degree Fahrenheit second per British thermal unit (th)'' : 'ThermalResistanceUnit' value)
    (attribute_usage ''degree Rankine'' : 'ThermodynamicTemperatureUnit', 'TemperatureDifferenceUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value))
      (default_ref_usage :>> 'ThermodynamicTemperatureUnit::quantityDimension', 'TemperatureDifferenceUnit::quantityDimension'
        (default_ref_usage :>> 'ThermodynamicTemperatureUnit::quantityDimension::quantityPowerFactors', 'TemperatureDifferenceUnit::quantityDimension::quantityPowerFactors')))
    (attribute_usage ''fathom (based on US survey foot)'' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''fluid ounce (US)'' : 'VolumeUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''foot'' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)))
    (attribute_usage ''foot (US survey)'' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''footcandle'' : 'IlluminanceUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''footlambert'' : 'LuminanceUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''foot of mercury, conventional'' : 'PressureUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''foot of water (39.2 °F)'' : 'PressureUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''foot of water, conventional'' : 'PressureUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''foot per hour'' : 'SpeedUnit' value)
    (attribute_usage ''foot per minute'' : 'SpeedUnit' value)
    (attribute_usage ''foot per second'' : 'SpeedUnit' value)
    (attribute_usage ''foot per second squared'' : 'AccelerationUnit' value)
    (attribute_usage ''foot poundal'' : 'EnergyUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''foot pound-force'' : 'EnergyUnit' value)
    (attribute_usage ''foot pound-force per hour'' : 'PowerUnit' value)
    (attribute_usage ''foot pound-force per minute'' : 'PowerUnit' value)
    (attribute_usage ''foot pound-force per second'' : 'PowerUnit' value)
    (attribute_usage ''foot to the fourth power'' : 'SecondAxialMomentOfAreaUnit' value)
    (attribute_usage ''gallon (US)'' : 'VolumeUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''gallon (US) per day'' : 'VolumeFlowRateUnit' value)
    (line_comment)
    (attribute_usage ''gallon (US) per minute (gpm)'' : 'VolumeFlowRateUnit' value)
    (attribute_usage ''gill (US)'' : 'VolumeUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''grain'' : 'MassUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)))
    (attribute_usage ''grain per gallon (US)'' : 'MassDensityUnit' value)
    (attribute_usage ''horsepower (550 ft*lbf/s)'' : 'PowerUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''horsepower (boiler)'' : 'PowerUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''horsepower (electric)'' : 'PowerUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)))
    (attribute_usage ''horsepower (water)'' : 'PowerUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''hundredweight (long, 112 lb)'' : 'MassUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''hundredweight (short, 100 lb)'' : 'MassUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''inch'' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)))
    (attribute_usage ''inch of mercury (32 °F)'' : 'PressureUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''inch of mercury (60 °F)'' : 'PressureUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''inch of mercury, conventional'' : 'PressureUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''inch of water (39.2 °F)'' : 'PressureUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''inch of water (60 °F)'' : 'PressureUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''inch of water, conventional'' : 'PressureUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''inch per second'' : 'SpeedUnit' value)
    (attribute_usage ''inch per second squared'' : 'AccelerationUnit' value)
    (attribute_usage ''inch to the fourth power'' : 'SecondAxialMomentOfAreaUnit' value)
    (attribute_usage ''kip (1 kip = 1000 lbf)'' : 'ForceUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''kip per square inch (ksi)'' : 'PressureUnit' value)
    (attribute_usage ''knot (nautical mile per hour)'' : 'SpeedUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (line_comment)
    (attribute_usage ''lumen per square foot'' : 'IlluminanceUnit' value)
    (attribute_usage ''microinch'' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)))
    (attribute_usage ''mil (0.001 in)'' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)))
    (attribute_usage ''mile'' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)))
    (attribute_usage ''mile (based on US survey foot)'' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''mile, nautical'' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)))
    (alias_member 'NM' for 'nmi')
    (line_comment)
    (line_comment)
    (attribute_usage ''mile per hour'' : 'SpeedUnit' value)
    (alias_member 'mph' for ''mi/h'')
    (attribute_usage ''mile per minute'' : 'SpeedUnit' value)
    (attribute_usage ''mile per second'' : 'SpeedUnit' value)
    (attribute_usage ''ohm circular-mil per foot'' : 'ResistivityUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''ounce (avoirdupois)'' : 'MassUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''ounce (US fluid)'' : 'VolumeUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''ounce (avoirdupois)-force'' : 'ForceUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''ounce (avoirdupois)-force inch'' : 'MomentOfForceUnit' value)
    (attribute_usage ''ounce (avoirdupois) per cubic inch'' : 'MassDensityUnit' value)
    (attribute_usage ''ounce (avoirdupois) per gallon (US)'' : 'MassDensityUnit' value)
    (attribute_usage ''ounce (avoirdupois) per square foot'' : 'SurfaceMassDensityUnit' value)
    (attribute_usage ''ounce (avoirdupois) per square inch'' : 'SurfaceMassDensityUnit' value)
    (attribute_usage ''ounce (avoirdupois) per square yard'' : 'SurfaceMassDensityUnit' value)
    (attribute_usage ''peck (US)'' : 'VolumeUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (line_comment)
    (line_comment)
    (line_comment)
    (line_comment)
    (attribute_usage ''pica (computer) (1/6 in)'' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''pica (printer′s)'' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''pint (US dry)'' : 'VolumeUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''pint (US liquid)'' : 'VolumeUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''point (computer) (1/72 in)'' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''point (printer′s)'' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''pound (avoirdupois)'' : 'MassUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''pound foot squared'' : 'MomentOfInertiaUnit' value)
    (attribute_usage ''pound-force'' : 'ForceUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''pound-force foot'' : 'MomentOfForceUnit' value)
    (attribute_usage ''pound-force foot per inch'' : 'ForceUnit' value)
    (attribute_usage ''pound-force inch'' : 'MomentOfForceUnit' value)
    (attribute_usage ''pound-force inch per inch'' : 'ForceUnit' value)
    (attribute_usage ''pound-force per foot'' : 'SurfaceTensionUnit' value)
    (attribute_usage ''pound-force per inch'' : 'SurfaceTensionUnit' value)
    (line_comment)
    (attribute_usage ''pound-force per square foot'' : 'PressureUnit' value)
    (attribute_usage ''pound-force per square inch'' : 'PressureUnit' value)
    (alias_member 'psi' for ''lbf/in²'')
    (attribute_usage ''pound-force second per square foot'' : 'DynamicViscosityUnit' value)
    (attribute_usage ''pound-force second per square inch'' : 'DynamicViscosityUnit' value)
    (attribute_usage ''pound inch squared'' : 'MomentOfInertiaUnit' value)
    (attribute_usage ''pound per cubic foot'' : 'MassDensityUnit' value)
    (attribute_usage ''pound per cubic inch'' : 'MassDensityUnit' value)
    (attribute_usage ''pound per cubic yard'' : 'MassDensityUnit' value)
    (attribute_usage ''pound per foot'' : 'LinearMassDensityUnit' value)
    (attribute_usage ''pound per foot hour'' : 'DynamicViscosityUnit' value)
    (attribute_usage ''pound per foot second'' : 'DynamicViscosityUnit' value)
    (attribute_usage ''pound per gallon (US)'' : 'MassDensityUnit' value)
    (line_comment)
    (attribute_usage ''pound per hour'' : 'MassFlowRateUnit' value)
    (attribute_usage ''pound per inch'' : 'LinearMassDensityUnit' value)
    (attribute_usage ''pound per minute'' : 'MassFlowRateUnit' value)
    (attribute_usage ''pound per second'' : 'MassFlowRateUnit' value)
    (attribute_usage ''pound per square foot'' : 'SurfaceMassDensityUnit' value)
    (attribute_usage ''pound per square inch (not pound-force)'' : 'SurfaceMassDensityUnit' value)
    (attribute_usage ''pound per yard'' : 'LinearMassDensityUnit' value)
    (attribute_usage ''pound-force per square inch (psi)'' : 'PressureUnit' value)
    (attribute_usage ''quad (10^15 Btu_IT)'' : 'EnergyUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''quart (US dry)'' : 'VolumeUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''quart (US liquid)'' : 'VolumeUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''rod (based on US survey foot)'' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''slug'' : 'MassUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''slug per cubic foot'' : 'MassDensityUnit' value)
    (attribute_usage ''slug per foot second'' : 'DynamicViscosityUnit' value)
    (attribute_usage ''square foot'' : 'AreaUnit' value)
    (attribute_usage ''square foot per hour'' : 'KinematicViscosityUnit' value)
    (attribute_usage ''square foot per second'' : 'KinematicViscosityUnit' value)
    (attribute_usage ''square inch'' : 'AreaUnit' value)
    (attribute_usage ''square mile'' : 'AreaUnit' value)
    (attribute_usage ''square mile (based on US survey foot)'' : 'AreaUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''square yard'' : 'AreaUnit' value)
    (attribute_usage ''tablespoon'' : 'VolumeUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''teaspoon'' : 'VolumeUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''therm (EC)'' : 'EnergyUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)))
    (attribute_usage ''therm (US)'' : 'EnergyUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)))
    (attribute_usage ''ton, assay'' : 'MassUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''ton-force (2000 lbf)'' : 'ForceUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''ton, long (2240 lb)'' : 'MassUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''ton, long, per cubic yard'' : 'MassDensityUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''ton of refrigeration (12 000 Btu_IT/h)'' : 'PowerUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''ton, register'' : 'VolumeUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''ton, short (2000 lb)'' : 'MassUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''ton, short, per cubic yard'' : 'MassDensityUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''ton, short, per hour'' : 'MassFlowRateUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''unit pole'' : 'MagneticFluxUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''watt per square inch'' : 'DensityOfHeatFlowRateUnit' value)
    (attribute_usage ''yard'' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)))
    (attribute_usage ''degree fahrenheit (absolute temperature scale)'' : 'IntervalScale'
      (documentation)
      (default_ref_usage :>> 'unit' value)
      (attribute_usage private 'temperatureWaterAtFreezingPointInF' : 'DefinitionalQuantityValue'
        (default_ref_usage :>> 'num' value)
        (default_ref_usage :>> 'definition' value))
      (attribute_usage private 'fahrenheitToCelsiusScaleMapping' : 'QuantityValueMapping'
        (default_ref_usage :>> 'mappedQuantityValue' value)
        (default_ref_usage :>> 'referenceQuantityValue' value))
      (attribute_usage :>> 'definitionalQuantityValues' value)
      (attribute_usage :>> 'quantityValueMapping' value)
      (comment)
      (attribute_usage private 'zeroDegreeFahrenheitInKelvin' : 'ThermodynamicTemperatureValue' value)
      (attribute_usage 'zeroDegreeFahrenheitToKelvinShift' : 'CoordinateFramePlacement' :>> 'transformation'
        (default_ref_usage :>> 'source' value)
        (default_ref_usage :>> 'origin' value)))))
~~~
# FORMAT
~~~sysml
standard library package <USCU> USCustomaryUnits {
    doc /*
	 * Measurement unit declarations generated from NIST SP811 Appendix B
	 *
	 * See https://www.nist.gov/pml/special-publication-811/nist-guide-si-appendix-b-conversion-factors/nist-guide-si-appendix-b8
	 */

    private import MeasurementReferences::*;
    public import ISQ::*;
    private import SI::*;

    attribute 'acre (based on US survey foot)' : AreaUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m^2;
            :>> conversionFactor = 4.046873E+03;
            :>> isExact = false;
        }
    }
    attribute 'acre foot (based on US survey foot)' : VolumeUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m^3;
            :>> conversionFactor = 1.233489E+03;
            :>> isExact = false;
        }
    }
    attribute <bbl> 'barrel (for petroleum, 42 gallons (US))' : VolumeUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m^3;
            :>> conversionFactor = 1.589873E-01;
            :>> isExact = false;
        }
    }
    attribute <Btu_IT> 'British thermal unit (IT)' : EnergyUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = J;
            :>> conversionFactor = 1.055056E+03;
            :>> isExact = false;
        }
    }
    alias Btu for Btu_IT;
    attribute <Btu_th> 'British thermal unit (th)' : EnergyUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = J;
            :>> conversionFactor = 1.054350E+03;
            :>> isExact = false;
        }
    }
    attribute <Btu_mean> 'British thermal unit (mean)' : EnergyUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = J;
            :>> conversionFactor = 1.05587E+03;
            :>> isExact = false;
        }
    }
    attribute <'Btu_39°F'> 'British thermal unit (39 °F)' : EnergyUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = J;
            :>> conversionFactor = 1.05967E+03;
            :>> isExact = false;
        }
    }
    attribute <'Btu_59°F'> 'British thermal unit (59 °F)' : EnergyUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = J;
            :>> conversionFactor = 1.05480E+03;
            :>> isExact = false;
        }
    }
    attribute <'Btu_60°F'> 'British thermal unit (60 °F)' : EnergyUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = J;
            :>> conversionFactor = 1.05468E+03;
            :>> isExact = false;
        }
    }
    attribute <'Btu_IT⋅ft/(h⋅ft²⋅°F)'> 'British thermal unit (IT) foot per hour square foot degree Fahrenheit' : ThermalConductivityUnit = Btu_IT*ft/(h*ft^2*'°F');
    attribute <'Btu_th⋅ft/(h⋅ft²⋅°F)'> 'British thermal unit (th) foot per hour square foot degree Fahrenheit' : ThermalConductivityUnit = Btu_th*ft/(h*ft^2*'°F');
    attribute <'Btu_IT⋅in/(h⋅ft²⋅°F)'> 'British thermal unit (IT) inch per hour square foot degree Fahrenheit' : ThermalConductivityUnit = Btu_IT*'in'/(h*ft^2*'°F');
    attribute <'Btu_th⋅in/(h⋅ft²⋅°F)'> 'British thermal unit (th) inch per hour square foot degree Fahrenheit' : ThermalConductivityUnit = Btu_th*'in'/(h*ft^2*'°F');
    attribute <'Btu_IT⋅in/(s⋅ft²⋅°F)'> 'British thermal unit (IT) inch per second square foot degree Fahrenheit' : ThermalConductivityUnit = Btu_IT*'in'/(s*ft^2*'°F');
    attribute <'Btu_th⋅in/(s⋅ft²⋅°F)'> 'British thermal unit (th) inch per second square foot degree Fahrenheit' : ThermalConductivityUnit = Btu_th*'in'/(s*ft^2*'°F');
    //attribute <'Btu_IT/ft³'> 'British thermal unit (IT) per cubic foot' : EnergyDensityUnit = Btu_IT/ft^3;
    //attribute <'Btu_th/ft³'> 'British thermal unit (th) per cubic foot' : EnergyDensityUnit = Btu_th/ft^3;
    attribute <'Btu_IT/°F'> 'British thermal unit (IT) per degree Fahrenheit' : HeatCapacityUnit = Btu_IT/'°F';
    attribute <'Btu_th/°F'> 'British thermal unit (th) per degree Fahrenheit' : HeatCapacityUnit = Btu_th/'°F';
    attribute <'Btu_IT/°R'> 'British thermal unit (IT) per degree Rankine' : HeatCapacityUnit = Btu_IT/'°R';
    attribute <'Btu_th/°R'> 'British thermal unit (th) per degree Rankine' : HeatCapacityUnit = Btu_th/'°R';
    attribute <'Btu_IT/h'> 'British thermal unit (IT) per hour' : PowerUnit = Btu_IT/h;
    attribute <'Btu_th/h'> 'British thermal unit (th) per hour' : PowerUnit = Btu_th/h;
    attribute <'Btu_IT/(h⋅ft²⋅°F)'> 'British thermal unit (IT) per hour square foot degree Fahrenheit' : CoefficientOfHeatTransferUnit = Btu_IT/(h*ft^2*'°F');
    attribute <'Btu_th/(h⋅ft²⋅°F)'> 'British thermal unit (th) per hour square foot degree Fahrenheit' : CoefficientOfHeatTransferUnit = Btu_th/(h*ft^2*'°F');
    attribute <'Btu_th/min'> 'British thermal unit (th) per minute' : PowerUnit = Btu_th/min;
    attribute <'Btu_IT/lb'> 'British thermal unit (IT) per pound' : SpecificEnergyUnit = Btu_IT/lb;
    attribute <'Btu_th/lb'> 'British thermal unit (th) per pound' : SpecificEnergyUnit = Btu_th/lb;
    attribute <'Btu_IT/(lb⋅°F)'> 'British thermal unit (IT) per pound degree Fahrenheit' : SpecificHeatCapacityUnit = Btu_IT/(lb*'°F');
    attribute <'Btu_th/(lb⋅°F)'> 'British thermal unit (th) per pound degree Fahrenheit' : SpecificHeatCapacityUnit = Btu_th/(lb*'°F');
    attribute <'Btu_IT/(lb⋅°R)'> 'British thermal unit (IT) per pound degree Rankine' : SpecificHeatCapacityUnit = Btu_IT/(lb*'°R');
    attribute <'Btu_th/(lb⋅°R)'> 'British thermal unit (th) per pound degree Rankine' : SpecificHeatCapacityUnit = Btu_th/(lb*'°R');
    attribute <'Btu_IT/s'> 'British thermal unit (IT) per second' : PowerUnit = Btu_IT/s;
    attribute <'Btu_th/s'> 'British thermal unit (th) per second' : PowerUnit = Btu_th/s;
    attribute <'Btu_IT/(s⋅ft²⋅°F)'> 'British thermal unit (IT) per second square foot degree Fahrenheit' : CoefficientOfHeatTransferUnit = Btu_IT/(s*ft^2*'°F');
    attribute <'Btu_th/(s⋅ft²⋅°F)'> 'British thermal unit (th) per second square foot degree Fahrenheit' : CoefficientOfHeatTransferUnit = Btu_th/(s*ft^2*'°F');
    //attribute <'Btu_IT/ft²'> 'British thermal unit (IT) per square foot' : SurfaceHeatDensityUnit = Btu_IT/ft^2;
    //attribute <'Btu_th/ft²'> 'British thermal unit (th) per square foot' : SurfaceHeatDensityUnit = Btu_th/ft^2;
    attribute <'Btu_IT/(ft²⋅h)'> 'British thermal unit (IT) per square foot hour' : DensityOfHeatFlowRateUnit = Btu_IT/(ft^2*h);
    attribute <'Btu_th/(ft²⋅h)'> 'British thermal unit (th) per square foot hour' : DensityOfHeatFlowRateUnit = Btu_th/(ft^2*h);
    attribute <'Btu_th/(ft²⋅min)'> 'British thermal unit (th) per square foot minute' : DensityOfHeatFlowRateUnit = Btu_th/(ft^2*min);
    attribute <'Btu_IT/(ft²⋅s)'> 'British thermal unit (IT) per square foot second' : DensityOfHeatFlowRateUnit = Btu_IT/(ft^2*s);
    attribute <'Btu_th/(ft²⋅s)'> 'British thermal unit (th) per square foot second' : DensityOfHeatFlowRateUnit = Btu_th/(ft^2*s);
    attribute <'Btu_th/(in²⋅s)'> 'British thermal unit (th) per square inch second' : DensityOfHeatFlowRateUnit = Btu_th/('in'^2*s);
    attribute <bu> 'bushel (US)' : VolumeUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m^3;
            :>> conversionFactor = 3.523907E-02;
            :>> isExact = false;
        }
    }
    attribute <'cd/in²'> 'candela per square inch' : LuminanceUnit = cd/'in'^2;
    attribute <ch> 'chain (based on US survey foot)' : LengthUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m;
            :>> conversionFactor = 2.011684E+01;
            :>> isExact = false;
        }
    }
    attribute 'circular mil' : AreaUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m^2;
            :>> conversionFactor = 5.067075E-10;
            :>> isExact = false;
        }
    }
    attribute 'clo' : ThermalInsulanceUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m^2*K/W;
            :>> conversionFactor = 1.55E-01;
            :>> isExact = false;
        }
    }
    attribute 'cord (128 ft^3)' : VolumeUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m^3;
            :>> conversionFactor = 3.624556E+00;
            :>> isExact = false;
        }
    }
    attribute <'ft³'> 'cubic foot' : VolumeUnit = ft^3;
    attribute <'ft³/min'> 'cubic foot per minute' : VolumeFlowRateUnit = ft^3/min;
    attribute <'ft³/s'> 'cubic foot per second' : VolumeFlowRateUnit = ft^3/s;
    attribute <'in³'> 'cubic inch' : VolumeUnit = 'in'^3;
    attribute <'in³/min'> 'cubic inch per minute' : VolumeFlowRateUnit = 'in'^3/min;
    attribute <'mi³'> 'cubic mile' : VolumeUnit = mi^3;
    attribute <'yd³'> 'cubic yard' : VolumeUnit = yd^3;
    attribute <'yd³/min'> 'cubic yard per minute' : VolumeFlowRateUnit = yd^3/min;
    attribute 'cup (US)' : VolumeUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m^3;
            :>> conversionFactor = 2.365882E-04;
            :>> isExact = false;
        }
    }
    attribute <'°F'> 'degree Fahrenheit (temperature difference)' : TemperatureDifferenceUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = K;
            :>> conversionFactor = 5/9;
            :>> isExact = true;
        }
    }
    attribute <'°F⋅h/Btu_IT'> 'degree Fahrenheit hour per British thermal unit (IT)' : ThermalResistanceUnit = '°F'*h/Btu_IT;
    attribute <'°F⋅h/Btu_th'> 'degree Fahrenheit hour per British thermal unit (th)' : ThermalResistanceUnit = '°F'*h/Btu_th;
    attribute <'°F⋅h⋅ft²/Btu_IT'> 'degree Fahrenheit hour square foot per British thermal unit (IT)' : ThermalInsulanceUnit = '°F'*h*ft^2/Btu_IT;
    attribute <'°F⋅h⋅ft²/Btu_th'> 'degree Fahrenheit hour square foot per British thermal unit (th)' : ThermalInsulanceUnit = '°F'*h*ft^2/Btu_th;
    //attribute <'°F⋅h⋅ft²/(Btu_IT⋅in)'> 'degree Fahrenheit hour square foot per British thermal unit (IT) inch' : ThermalResistivityUnit = '°F'*h*ft^2/(Btu_IT*'in');
    //attribute <'°F⋅h⋅ft²/(Btu_th⋅in)'> 'degree Fahrenheit hour square foot per British thermal unit (th) inch' : ThermalResistivityUnit = '°F'*h*ft^2/(Btu_th*'in');
    attribute <'°F⋅s/Btu_IT'> 'degree Fahrenheit second per British thermal unit (IT)' : ThermalResistanceUnit = '°F'*s/Btu_IT;
    attribute <'°F⋅s/Btu_th'> 'degree Fahrenheit second per British thermal unit (th)' : ThermalResistanceUnit = '°F'*s/Btu_th;
    attribute <'°R'> 'degree Rankine' : ThermodynamicTemperatureUnit, TemperatureDifferenceUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = K;
            :>> conversionFactor = 5/9;
            :>> isExact = true;
        }
        :>> ThermodynamicTemperatureUnit::quantityDimension, TemperatureDifferenceUnit::quantityDimension {
            :>> ThermodynamicTemperatureUnit::quantityDimension::quantityPowerFactors, TemperatureDifferenceUnit::quantityDimension::quantityPowerFactors;
        }
    }
    attribute 'fathom (based on US survey foot)' : LengthUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m;
            :>> conversionFactor = 1.828804E+00;
            :>> isExact = false;
        }
    }
    attribute <floz> 'fluid ounce (US)' : VolumeUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m^3;
            :>> conversionFactor = 2.957353E-05;
            :>> isExact = false;
        }
    }
    attribute <ft> 'foot' : LengthUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m;
            :>> conversionFactor = 3.048E-01;
        }
    }
    attribute 'foot (US survey)' : LengthUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m;
            :>> conversionFactor = 3.048006E-01;
            :>> isExact = false;
        }
    }
    attribute 'footcandle' : IlluminanceUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = lx;
            :>> conversionFactor = 1.076391E+01;
            :>> isExact = false;
        }
    }
    attribute 'footlambert' : LuminanceUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = cd/m^2;
            :>> conversionFactor = 3.426259E+00;
            :>> isExact = false;
        }
    }
    attribute <ftHg> 'foot of mercury, conventional' : PressureUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = Pa;
            :>> conversionFactor = 4.063666E+04;
            :>> isExact = false;
        }
    }
    attribute 'foot of water (39.2 °F)' : PressureUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = Pa;
            :>> conversionFactor = 2.98898E+03;
            :>> isExact = false;
        }
    }
    attribute <ftH2O> 'foot of water, conventional' : PressureUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = Pa;
            :>> conversionFactor = 2.989067E+03;
            :>> isExact = false;
        }
    }
    attribute <'ft/h'> 'foot per hour' : SpeedUnit = ft/h;
    attribute <'ft/min'> 'foot per minute' : SpeedUnit = ft/min;
    attribute <'ft/s'> 'foot per second' : SpeedUnit = ft/s;
    attribute <'ft/s²'> 'foot per second squared' : AccelerationUnit = ft/s^2;
    attribute 'foot poundal' : EnergyUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = J;
            :>> conversionFactor = 4.214011E-02;
            :>> isExact = false;
        }
    }
    attribute <'ft⋅lbf'> 'foot pound-force' : EnergyUnit = ft*lbf;
    attribute <'ft⋅lbf/h'> 'foot pound-force per hour' : PowerUnit = ft*lbf/h;
    attribute <'ft⋅lbf/min'> 'foot pound-force per minute' : PowerUnit = ft*lbf/min;
    attribute <'ft⋅lbf/s'> 'foot pound-force per second' : PowerUnit = ft*lbf/s;
    attribute <'ft⁴'> 'foot to the fourth power' : SecondAxialMomentOfAreaUnit = ft^4;
    attribute <gal> 'gallon (US)' : VolumeUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m^3;
            :>> conversionFactor = 3.785412E-03;
            :>> isExact = false;
        }
    }
    attribute <'gal/d'> 'gallon (US) per day' : VolumeFlowRateUnit = gal/d;
    //attribute <'gal/(hp⋅h)'> 'gallon (US) per horsepower hour' : EnergySpecificVolumeUnit = gal/(hp*h);
    attribute <'gal/min'> 'gallon (US) per minute (gpm)' : VolumeFlowRateUnit = gal/min;
    attribute <gi> 'gill (US)' : VolumeUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m^3;
            :>> conversionFactor = 1.182941E-04;
            :>> isExact = false;
        }
    }
    attribute <gr> 'grain' : MassUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = kg;
            :>> conversionFactor = 6.479891E-05;
        }
    }
    attribute <'gr/gal'> 'grain per gallon (US)' : MassDensityUnit = gr/gal;
    attribute <hp> 'horsepower (550 ft*lbf/s)' : PowerUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = W;
            :>> conversionFactor = 7.456999E+02;
            :>> isExact = false;
        }
    }
    attribute 'horsepower (boiler)' : PowerUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = W;
            :>> conversionFactor = 9.80950E+03;
            :>> isExact = false;
        }
    }
    attribute 'horsepower (electric)' : PowerUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = W;
            :>> conversionFactor = 7.46E+02;
        }
    }
    attribute 'horsepower (water)' : PowerUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = W;
            :>> conversionFactor = 7.46043E+02;
            :>> isExact = false;
        }
    }
    attribute 'hundredweight (long, 112 lb)' : MassUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = kg;
            :>> conversionFactor = 5.080235E+01;
            :>> isExact = false;
        }
    }
    attribute 'hundredweight (short, 100 lb)' : MassUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = kg;
            :>> conversionFactor = 4.535924E+01;
            :>> isExact = false;
        }
    }
    attribute <'in'> 'inch' : LengthUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m;
            :>> conversionFactor = 2.54E-02;
        }
    }
    attribute 'inch of mercury (32 °F)' : PressureUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = Pa;
            :>> conversionFactor = 3.38638E+03;
            :>> isExact = false;
        }
    }
    attribute 'inch of mercury (60 °F)' : PressureUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = Pa;
            :>> conversionFactor = 3.37685E+03;
            :>> isExact = false;
        }
    }
    attribute <inHg> 'inch of mercury, conventional' : PressureUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = Pa;
            :>> conversionFactor = 3.386389E+03;
            :>> isExact = false;
        }
    }
    attribute 'inch of water (39.2 °F)' : PressureUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = Pa;
            :>> conversionFactor = 2.49082E+02;
            :>> isExact = false;
        }
    }
    attribute 'inch of water (60 °F)' : PressureUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = Pa;
            :>> conversionFactor = 2.4884E+02;
            :>> isExact = false;
        }
    }
    attribute <inH2O> 'inch of water, conventional' : PressureUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = Pa;
            :>> conversionFactor = 2.490889E+02;
            :>> isExact = false;
        }
    }
    attribute <'in/s'> 'inch per second' : SpeedUnit = 'in'/s;
    attribute <'in/s²'> 'inch per second squared' : AccelerationUnit = 'in'/s^2;
    attribute <'in⁴'> 'inch to the fourth power' : SecondAxialMomentOfAreaUnit = 'in'^4;
    attribute <kip> 'kip (1 kip = 1000 lbf)' : ForceUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = N;
            :>> conversionFactor = 4.448222E+03;
            :>> isExact = false;
        }
    }
    attribute <'kip/in²'> 'kip per square inch (ksi)' : PressureUnit = kip/'in'^2;
    attribute <knot> 'knot (nautical mile per hour)' : SpeedUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m/s;
            :>> conversionFactor = 5.144444E-01;
            :>> isExact = false;
        }
    }
    //attribute <'cal_th/cm²'> 'langley' : SurfaceHeatDensityUnit = cal_th/cm^2;
    attribute <'lm/ft²'> 'lumen per square foot' : IlluminanceUnit = lm/ft^2;
    attribute 'microinch' : LengthUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m;
            :>> conversionFactor = 2.54E-08;
        }
    }
    attribute <mil> 'mil (0.001 in)' : LengthUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m;
            :>> conversionFactor = 2.54E-05;
        }
    }
    attribute <mi> 'mile' : LengthUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m;
            :>> conversionFactor = 1.609344E+03;
        }
    }
    attribute 'mile (based on US survey foot)' : LengthUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m;
            :>> conversionFactor = 1.609347E+03;
            :>> isExact = false;
        }
    }
    attribute <nmi> 'mile, nautical' : LengthUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m;
            :>> conversionFactor = 1.852E+03;
        }
    }
    alias NM for nmi;
    //attribute <'mi/gal'> 'mile per gallon (US)' : FuelEconomyUnit = mi/gal;
    //alias mpg for 'mi/gal';
    attribute <'mi/h'> 'mile per hour' : SpeedUnit = mi/h;
    alias mph for 'mi/h';
    attribute <'mi/min'> 'mile per minute' : SpeedUnit = mi/min;
    attribute <'mi/s'> 'mile per second' : SpeedUnit = mi/s;
    attribute 'ohm circular-mil per foot' : ResistivityUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = 'Ω'*m;
            :>> conversionFactor = 1.662426E-09;
            :>> isExact = false;
        }
    }
    attribute <oz> 'ounce (avoirdupois)' : MassUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = kg;
            :>> conversionFactor = 2.834952E-02;
            :>> isExact = false;
        }
    }
    attribute 'ounce (US fluid)' : VolumeUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m^3;
            :>> conversionFactor = 2.957353E-05;
            :>> isExact = false;
        }
    }
    attribute <ozf> 'ounce (avoirdupois)-force' : ForceUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = N;
            :>> conversionFactor = 2.780139E-01;
            :>> isExact = false;
        }
    }
    attribute <'ozf⋅in'> 'ounce (avoirdupois)-force inch' : MomentOfForceUnit = ozf*'in';
    attribute <'oz/in³'> 'ounce (avoirdupois) per cubic inch' : MassDensityUnit = oz/'in'^3;
    attribute <'oz/gal'> 'ounce (avoirdupois) per gallon (US)' : MassDensityUnit = oz/gal;
    attribute <'oz/ft²'> 'ounce (avoirdupois) per square foot' : SurfaceMassDensityUnit = oz/ft^2;
    attribute <'oz/in²'> 'ounce (avoirdupois) per square inch' : SurfaceMassDensityUnit = oz/'in'^2;
    attribute <'oz/yd²'> 'ounce (avoirdupois) per square yard' : SurfaceMassDensityUnit = oz/yd^2;
    attribute <pk> 'peck (US)' : VolumeUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m^3;
            :>> conversionFactor = 8.809768E-03;
            :>> isExact = false;
        }
    }
    //attribute 'perm (0 °C)' : VapourTransmissionUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg/(Pa*s*m^2); :>> conversionFactor = 5.72135E-11; :>> isExact = false; } }
    //attribute 'perm (23 °C)' : VapourTransmissionUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg/(Pa*s*m^2); :>> conversionFactor = 5.74525E-11; :>> isExact = false; } }
    //attribute 'perm inch (0 °C)' : VapourTransmissionUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg/('Pa·s·m'); :>> conversionFactor = 1.45322E-12; :>> isExact = false; } }
    //attribute 'perm inch (23 °C)' : VapourTransmissionUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg/('Pa·s·m'); :>> conversionFactor = 1.45929E-12; :>> isExact = false; } }
    attribute <pica> 'pica (computer) (1/6 in)' : LengthUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m;
            :>> conversionFactor = 4.233333E-03;
            :>> isExact = false;
        }
    }
    attribute 'pica (printer′s)' : LengthUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m;
            :>> conversionFactor = 4.217518E-03;
            :>> isExact = false;
        }
    }
    attribute <drypt> 'pint (US dry)' : VolumeUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m^3;
            :>> conversionFactor = 5.506105E-04;
            :>> isExact = false;
        }
    }
    attribute <liqpt> 'pint (US liquid)' : VolumeUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m^3;
            :>> conversionFactor = 4.731765E-04;
            :>> isExact = false;
        }
    }
    attribute <pt> 'point (computer) (1/72 in)' : LengthUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m;
            :>> conversionFactor = 3.527778E-04;
            :>> isExact = false;
        }
    }
    attribute 'point (printer′s)' : LengthUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m;
            :>> conversionFactor = 3.514598E-04;
            :>> isExact = false;
        }
    }
    attribute <lb> 'pound (avoirdupois)' : MassUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = kg;
            :>> conversionFactor = 4.535924E-01;
            :>> isExact = false;
        }
    }
    attribute <'lb⋅ft²'> 'pound foot squared' : MomentOfInertiaUnit = lb*ft^2;
    attribute <lbf> 'pound-force' : ForceUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = N;
            :>> conversionFactor = 4.448222E+00;
            :>> isExact = false;
        }
    }
    attribute <'lbf⋅ft'> 'pound-force foot' : MomentOfForceUnit = lbf*ft;
    attribute <'lbf⋅ft/in'> 'pound-force foot per inch' : ForceUnit = lbf*ft/'in';
    attribute <'lbf⋅in'> 'pound-force inch' : MomentOfForceUnit = lbf*'in';
    attribute <'lbf⋅in/in'> 'pound-force inch per inch' : ForceUnit = lbf*'in'/'in';
    attribute <'lbf/ft'> 'pound-force per foot' : SurfaceTensionUnit = lbf/ft;
    attribute <'lbf/in'> 'pound-force per inch' : SurfaceTensionUnit = lbf/'in';
    //attribute 'pound-force per pound (lbf/lb) (thrust to mass ratio)' : ThrustToMassRatioUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = N/kg; :>> conversionFactor = 9.80665E+00; } }
    attribute <'lbf/ft²'> 'pound-force per square foot' : PressureUnit = lbf/ft^2;
    attribute <'lbf/in²'> 'pound-force per square inch' : PressureUnit = lbf/'in'^2;
    alias psi for 'lbf/in²';
    attribute <'lbf⋅s/ft²'> 'pound-force second per square foot' : DynamicViscosityUnit = lbf*s/ft^2;
    attribute <'lbf⋅s/in²'> 'pound-force second per square inch' : DynamicViscosityUnit = lbf*s/'in'^2;
    attribute <'lb⋅in²'> 'pound inch squared' : MomentOfInertiaUnit = lb*'in'^2;
    attribute <'lb/ft³'> 'pound per cubic foot' : MassDensityUnit = lb/ft^3;
    attribute <'lb/in³'> 'pound per cubic inch' : MassDensityUnit = lb/'in'^3;
    attribute <'lb/yd³'> 'pound per cubic yard' : MassDensityUnit = lb/yd^3;
    attribute <'lb/ft'> 'pound per foot' : LinearMassDensityUnit = lb/ft;
    attribute <'lb/(ft⋅h)'> 'pound per foot hour' : DynamicViscosityUnit = lb/(ft*h);
    attribute <'lb/(ft⋅s)'> 'pound per foot second' : DynamicViscosityUnit = lb/(ft*s);
    attribute <'lb/gal'> 'pound per gallon (US)' : MassDensityUnit = lb/gal;
    //attribute <'lb/(hp⋅h)'> 'pound per horsepower hour' : FuelConsumptionUnit = lb/(hp*h);
    attribute <'lb/h'> 'pound per hour' : MassFlowRateUnit = lb/h;
    attribute <'lb/in'> 'pound per inch' : LinearMassDensityUnit = lb/'in';
    attribute <'lb/min'> 'pound per minute' : MassFlowRateUnit = lb/min;
    attribute <'lb/s'> 'pound per second' : MassFlowRateUnit = lb/s;
    attribute <'lb/ft²'> 'pound per square foot' : SurfaceMassDensityUnit = lb/ft^2;
    attribute <'lb/in²'> 'pound per square inch (not pound-force)' : SurfaceMassDensityUnit = lb/'in'^2;
    attribute <'lb/yd'> 'pound per yard' : LinearMassDensityUnit = lb/yd;
    attribute 'pound-force per square inch (psi)' : PressureUnit = lbf/'in'^2;
    attribute 'quad (10^15 Btu_IT)' : EnergyUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = J;
            :>> conversionFactor = 1.055056E+18;
            :>> isExact = false;
        }
    }
    attribute <dryqt> 'quart (US dry)' : VolumeUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m^3;
            :>> conversionFactor = 1.101221E-03;
            :>> isExact = false;
        }
    }
    attribute <liqqt> 'quart (US liquid)' : VolumeUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m^3;
            :>> conversionFactor = 9.463529E-04;
            :>> isExact = false;
        }
    }
    attribute <rd> 'rod (based on US survey foot)' : LengthUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m;
            :>> conversionFactor = 5.029210E+00;
            :>> isExact = false;
        }
    }
    attribute <slug> 'slug' : MassUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = kg;
            :>> conversionFactor = 1.459390E+01;
            :>> isExact = false;
        }
    }
    attribute <'slug/ft³'> 'slug per cubic foot' : MassDensityUnit = slug/ft^3;
    attribute <'slug/(ft⋅s)'> 'slug per foot second' : DynamicViscosityUnit = slug/(ft*s);
    attribute <'ft²'> 'square foot' : AreaUnit = ft^2;
    attribute <'ft²/h'> 'square foot per hour' : KinematicViscosityUnit = ft^2/h;
    attribute <'ft²/s'> 'square foot per second' : KinematicViscosityUnit = ft^2/s;
    attribute <'in²'> 'square inch' : AreaUnit = 'in'^2;
    attribute <'mi²'> 'square mile' : AreaUnit = mi^2;
    attribute 'square mile (based on US survey foot)' : AreaUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m^2;
            :>> conversionFactor = 2.589998E+06;
            :>> isExact = false;
        }
    }
    attribute <'yd²'> 'square yard' : AreaUnit = yd^2;
    attribute 'tablespoon' : VolumeUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m^3;
            :>> conversionFactor = 1.478676E-05;
            :>> isExact = false;
        }
    }
    attribute 'teaspoon' : VolumeUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m^3;
            :>> conversionFactor = 4.928922E-06;
            :>> isExact = false;
        }
    }
    attribute 'therm (EC)' : EnergyUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = J;
            :>> conversionFactor = 1.05506E+08;
        }
    }
    attribute 'therm (US)' : EnergyUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = J;
            :>> conversionFactor = 1.054804E+08;
        }
    }
    attribute <AT> 'ton, assay' : MassUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = kg;
            :>> conversionFactor = 2.916667E-02;
            :>> isExact = false;
        }
    }
    attribute 'ton-force (2000 lbf)' : ForceUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = N;
            :>> conversionFactor = 8.896443E+03;
            :>> isExact = false;
        }
    }
    attribute 'ton, long (2240 lb)' : MassUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = kg;
            :>> conversionFactor = 1.016047E+03;
            :>> isExact = false;
        }
    }
    attribute 'ton, long, per cubic yard' : MassDensityUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = kg/m^3;
            :>> conversionFactor = 1.328939E+03;
            :>> isExact = false;
        }
    }
    attribute 'ton of refrigeration (12 000 Btu_IT/h)' : PowerUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = W;
            :>> conversionFactor = 3.516853E+03;
            :>> isExact = false;
        }
    }
    attribute 'ton, register' : VolumeUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m^3;
            :>> conversionFactor = 2.831685E+00;
            :>> isExact = false;
        }
    }
    attribute 'ton, short (2000 lb)' : MassUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = kg;
            :>> conversionFactor = 9.071847E+02;
            :>> isExact = false;
        }
    }
    attribute 'ton, short, per cubic yard' : MassDensityUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = kg/m^3;
            :>> conversionFactor = 1.186553E+03;
            :>> isExact = false;
        }
    }
    attribute 'ton, short, per hour' : MassFlowRateUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = kg/s;
            :>> conversionFactor = 2.519958E-01;
            :>> isExact = false;
        }
    }
    attribute 'unit pole' : MagneticFluxUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = Wb;
            :>> conversionFactor = 1.256637E-07;
            :>> isExact = false;
        }
    }
    attribute <'W/in²'> 'watt per square inch' : DensityOfHeatFlowRateUnit = W/'in'^2;
    attribute <yd> 'yard' : LengthUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m;
            :>> conversionFactor = 9.144E-01;
        }
    }

    attribute <'°F_abs'> 'degree fahrenheit (absolute temperature scale)' : IntervalScale {
        doc /*
		 * degree Fahrenheit interval scale for absolute (thermodynamic) temperature quantities
		 *
	     * The interval scale is defined with an explicit transformation with respect to 
	     * the kelvin thermodynamic temperature scale that specifies the zero shift.
		 */

        :>> unit = '°F';
        private attribute temperatureWaterAtFreezingPointInF : DefinitionalQuantityValue {
            :>> num = 32.0;
            :>> definition = "temperature in degree Fahrenheit of pure water at freezing point";
        }
        private attribute fahrenheitToCelsiusScaleMapping : QuantityValueMapping {
            :>> mappedQuantityValue = temperatureWaterAtFreezingPointInF;
            :>> referenceQuantityValue = '°C_abs'.temperatureWaterAtFreezingPointInC;
        }
        attribute :>> definitionalQuantityValues = temperatureWaterAtFreezingPointInF;
        attribute :>> quantityValueMapping = fahrenheitToCelsiusScaleMapping;

        /* CoordinateFramePlacement (zero shift) w.r.t. the kelvin thermodynamic temperature scale */
        private attribute zeroDegreeFahrenheitInKelvin : ThermodynamicTemperatureValue = 229835/900 [K];
        attribute zeroDegreeFahrenheitToKelvinShift : CoordinateFramePlacement :>> transformation {
            :>> source = K;
            :>> origin = zeroDegreeFahrenheitInKelvin;
        }
    }
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "USCustomaryUnits"))) (name "USCustomaryUnits") (declared-name "USCustomaryUnits")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "USCustomaryUnits::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "USCustomaryUnits::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "USCustomaryUnits::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (39 °F)"))) (name "British thermal unit (39 °F)") (declared-name "British thermal unit (39 °F)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (39 °F)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (39 °F)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (59 °F)"))) (name "British thermal unit (59 °F)") (declared-name "British thermal unit (59 °F)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (59 °F)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (59 °F)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (60 °F)"))) (name "British thermal unit (60 °F)") (declared-name "British thermal unit (60 °F)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (60 °F)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (60 °F)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT)"))) (name "British thermal unit (IT)") (declared-name "British thermal unit (IT)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) foot per hour square foot degree Fahrenheit"))) (name "British thermal unit (IT) foot per hour square foot degree Fahrenheit") (declared-name "British thermal unit (IT) foot per hour square foot degree Fahrenheit") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "Btu_IT")) (expression (kind "featureReference") (reference "ft")))) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "h")) (expression (kind "featureReference") (reference "ft")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "°F")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) foot per hour square foot degree Fahrenheit"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) inch per hour square foot degree Fahrenheit"))) (name "British thermal unit (IT) inch per hour square foot degree Fahrenheit") (declared-name "British thermal unit (IT) inch per hour square foot degree Fahrenheit") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "Btu_IT")) (expression (kind "featureReference") (reference "in")))) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "h")) (expression (kind "featureReference") (reference "ft")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "°F")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) inch per hour square foot degree Fahrenheit"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) inch per second square foot degree Fahrenheit"))) (name "British thermal unit (IT) inch per second square foot degree Fahrenheit") (declared-name "British thermal unit (IT) inch per second square foot degree Fahrenheit") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "Btu_IT")) (expression (kind "featureReference") (reference "in")))) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "s")) (expression (kind "featureReference") (reference "ft")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "°F")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) inch per second square foot degree Fahrenheit"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per degree Fahrenheit"))) (name "British thermal unit (IT) per degree Fahrenheit") (declared-name "British thermal unit (IT) per degree Fahrenheit") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_IT")) (expression (kind "featureReference") (reference "°F")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per degree Fahrenheit"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per degree Rankine"))) (name "British thermal unit (IT) per degree Rankine") (declared-name "British thermal unit (IT) per degree Rankine") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_IT")) (expression (kind "featureReference") (reference "°R")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per degree Rankine"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per hour"))) (name "British thermal unit (IT) per hour") (declared-name "British thermal unit (IT) per hour") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_IT")) (expression (kind "featureReference") (reference "h")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per hour"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per hour square foot degree Fahrenheit"))) (name "British thermal unit (IT) per hour square foot degree Fahrenheit") (declared-name "British thermal unit (IT) per hour square foot degree Fahrenheit") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_IT")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "h")) (expression (kind "featureReference") (reference "ft")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "°F")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per hour square foot degree Fahrenheit"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per pound"))) (name "British thermal unit (IT) per pound") (declared-name "British thermal unit (IT) per pound") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_IT")) (expression (kind "featureReference") (reference "lb")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per pound"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per pound degree Fahrenheit"))) (name "British thermal unit (IT) per pound degree Fahrenheit") (declared-name "British thermal unit (IT) per pound degree Fahrenheit") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_IT")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "lb")) (expression (kind "featureReference") (reference "°F")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per pound degree Fahrenheit"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per pound degree Rankine"))) (name "British thermal unit (IT) per pound degree Rankine") (declared-name "British thermal unit (IT) per pound degree Rankine") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_IT")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "lb")) (expression (kind "featureReference") (reference "°R")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per pound degree Rankine"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per second"))) (name "British thermal unit (IT) per second") (declared-name "British thermal unit (IT) per second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_IT")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per second square foot degree Fahrenheit"))) (name "British thermal unit (IT) per second square foot degree Fahrenheit") (declared-name "British thermal unit (IT) per second square foot degree Fahrenheit") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_IT")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "s")) (expression (kind "featureReference") (reference "ft")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "°F")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per second square foot degree Fahrenheit"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per square foot hour"))) (name "British thermal unit (IT) per square foot hour") (declared-name "British thermal unit (IT) per square foot hour") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_IT")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "h")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per square foot hour"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per square foot second"))) (name "British thermal unit (IT) per square foot second") (declared-name "British thermal unit (IT) per square foot second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_IT")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "s")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per square foot second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (mean)"))) (name "British thermal unit (mean)") (declared-name "British thermal unit (mean)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (mean)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (mean)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th)"))) (name "British thermal unit (th)") (declared-name "British thermal unit (th)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) foot per hour square foot degree Fahrenheit"))) (name "British thermal unit (th) foot per hour square foot degree Fahrenheit") (declared-name "British thermal unit (th) foot per hour square foot degree Fahrenheit") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "Btu_th")) (expression (kind "featureReference") (reference "ft")))) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "h")) (expression (kind "featureReference") (reference "ft")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "°F")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) foot per hour square foot degree Fahrenheit"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) inch per hour square foot degree Fahrenheit"))) (name "British thermal unit (th) inch per hour square foot degree Fahrenheit") (declared-name "British thermal unit (th) inch per hour square foot degree Fahrenheit") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "Btu_th")) (expression (kind "featureReference") (reference "in")))) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "h")) (expression (kind "featureReference") (reference "ft")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "°F")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) inch per hour square foot degree Fahrenheit"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) inch per second square foot degree Fahrenheit"))) (name "British thermal unit (th) inch per second square foot degree Fahrenheit") (declared-name "British thermal unit (th) inch per second square foot degree Fahrenheit") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "Btu_th")) (expression (kind "featureReference") (reference "in")))) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "s")) (expression (kind "featureReference") (reference "ft")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "°F")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) inch per second square foot degree Fahrenheit"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per degree Fahrenheit"))) (name "British thermal unit (th) per degree Fahrenheit") (declared-name "British thermal unit (th) per degree Fahrenheit") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_th")) (expression (kind "featureReference") (reference "°F")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per degree Fahrenheit"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per degree Rankine"))) (name "British thermal unit (th) per degree Rankine") (declared-name "British thermal unit (th) per degree Rankine") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_th")) (expression (kind "featureReference") (reference "°R")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per degree Rankine"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per hour"))) (name "British thermal unit (th) per hour") (declared-name "British thermal unit (th) per hour") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_th")) (expression (kind "featureReference") (reference "h")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per hour"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per hour square foot degree Fahrenheit"))) (name "British thermal unit (th) per hour square foot degree Fahrenheit") (declared-name "British thermal unit (th) per hour square foot degree Fahrenheit") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_th")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "h")) (expression (kind "featureReference") (reference "ft")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "°F")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per hour square foot degree Fahrenheit"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per minute"))) (name "British thermal unit (th) per minute") (declared-name "British thermal unit (th) per minute") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_th")) (expression (kind "featureReference") (reference "min")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per minute"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per pound"))) (name "British thermal unit (th) per pound") (declared-name "British thermal unit (th) per pound") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_th")) (expression (kind "featureReference") (reference "lb")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per pound"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per pound degree Fahrenheit"))) (name "British thermal unit (th) per pound degree Fahrenheit") (declared-name "British thermal unit (th) per pound degree Fahrenheit") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_th")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "lb")) (expression (kind "featureReference") (reference "°F")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per pound degree Fahrenheit"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per pound degree Rankine"))) (name "British thermal unit (th) per pound degree Rankine") (declared-name "British thermal unit (th) per pound degree Rankine") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_th")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "lb")) (expression (kind "featureReference") (reference "°R")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per pound degree Rankine"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per second"))) (name "British thermal unit (th) per second") (declared-name "British thermal unit (th) per second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_th")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per second square foot degree Fahrenheit"))) (name "British thermal unit (th) per second square foot degree Fahrenheit") (declared-name "British thermal unit (th) per second square foot degree Fahrenheit") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_th")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "s")) (expression (kind "featureReference") (reference "ft")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "°F")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per second square foot degree Fahrenheit"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per square foot hour"))) (name "British thermal unit (th) per square foot hour") (declared-name "British thermal unit (th) per square foot hour") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_th")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "h")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per square foot hour"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per square foot minute"))) (name "British thermal unit (th) per square foot minute") (declared-name "British thermal unit (th) per square foot minute") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_th")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "min")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per square foot minute"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per square foot second"))) (name "British thermal unit (th) per square foot second") (declared-name "British thermal unit (th) per square foot second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_th")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "s")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per square foot second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per square inch second"))) (name "British thermal unit (th) per square inch second") (declared-name "British thermal unit (th) per square inch second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Btu_th")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "in")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "s")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per square inch second"))) (role feature-value))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "USCustomaryUnits::Btu"))) (name "Btu") (declared-name "Btu"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "USCustomaryUnits::NM"))) (name "NM") (declared-name "NM"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "USCustomaryUnits::_documentation"))) (name ""))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::acre (based on US survey foot)"))) (name "acre (based on US survey foot)") (declared-name "acre (based on US survey foot)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::acre (based on US survey foot)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::acre (based on US survey foot)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::acre foot (based on US survey foot)"))) (name "acre foot (based on US survey foot)") (declared-name "acre foot (based on US survey foot)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::acre foot (based on US survey foot)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::acre foot (based on US survey foot)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::barrel (for petroleum, 42 gallons (US))"))) (name "barrel (for petroleum, 42 gallons (US))") (declared-name "barrel (for petroleum, 42 gallons (US))") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::barrel (for petroleum, 42 gallons (US))::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::barrel (for petroleum, 42 gallons (US))")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::bushel (US)"))) (name "bushel (US)") (declared-name "bushel (US)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::bushel (US)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::bushel (US)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::candela per square inch"))) (name "candela per square inch") (declared-name "candela per square inch") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "cd")) (expression (kind "featureReference") (reference "in")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::candela per square inch"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::chain (based on US survey foot)"))) (name "chain (based on US survey foot)") (declared-name "chain (based on US survey foot)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::chain (based on US survey foot)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::chain (based on US survey foot)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::circular mil"))) (name "circular mil") (declared-name "circular mil") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::circular mil::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::circular mil")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::clo"))) (name "clo") (declared-name "clo") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::clo::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::clo")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::cord (128 ft^3)"))) (name "cord (128 ft^3)") (declared-name "cord (128 ft^3)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::cord (128 ft^3)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::cord (128 ft^3)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::cubic foot"))) (name "cubic foot") (declared-name "cubic foot") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "integerLiteral") (literal 3)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::cubic foot"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::cubic foot per minute"))) (name "cubic foot per minute") (declared-name "cubic foot per minute") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "integerLiteral") (literal 3)))) (expression (kind "featureReference") (reference "min")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::cubic foot per minute"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::cubic foot per second"))) (name "cubic foot per second") (declared-name "cubic foot per second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "integerLiteral") (literal 3)))) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::cubic foot per second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::cubic inch"))) (name "cubic inch") (declared-name "cubic inch") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "in")) (expression (kind "integerLiteral") (literal 3)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::cubic inch"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::cubic inch per minute"))) (name "cubic inch per minute") (declared-name "cubic inch per minute") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "in")) (expression (kind "integerLiteral") (literal 3)))) (expression (kind "featureReference") (reference "min")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::cubic inch per minute"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::cubic mile"))) (name "cubic mile") (declared-name "cubic mile") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "mi")) (expression (kind "integerLiteral") (literal 3)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::cubic mile"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::cubic yard"))) (name "cubic yard") (declared-name "cubic yard") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "yd")) (expression (kind "integerLiteral") (literal 3)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::cubic yard"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::cubic yard per minute"))) (name "cubic yard per minute") (declared-name "cubic yard per minute") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "yd")) (expression (kind "integerLiteral") (literal 3)))) (expression (kind "featureReference") (reference "min")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::cubic yard per minute"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::cup (US)"))) (name "cup (US)") (declared-name "cup (US)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::cup (US)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::cup (US)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit (temperature difference)"))) (name "degree Fahrenheit (temperature difference)") (declared-name "degree Fahrenheit (temperature difference)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit (temperature difference)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit (temperature difference)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit hour per British thermal unit (IT)"))) (name "degree Fahrenheit hour per British thermal unit (IT)") (declared-name "degree Fahrenheit hour per British thermal unit (IT)") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "°F")) (expression (kind "featureReference") (reference "h")))) (expression (kind "featureReference") (reference "Btu_IT")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit hour per British thermal unit (IT)"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit hour per British thermal unit (th)"))) (name "degree Fahrenheit hour per British thermal unit (th)") (declared-name "degree Fahrenheit hour per British thermal unit (th)") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "°F")) (expression (kind "featureReference") (reference "h")))) (expression (kind "featureReference") (reference "Btu_th")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit hour per British thermal unit (th)"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit hour square foot per British thermal unit (IT)"))) (name "degree Fahrenheit hour square foot per British thermal unit (IT)") (declared-name "degree Fahrenheit hour square foot per British thermal unit (IT)") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "°F")) (expression (kind "featureReference") (reference "h")))) (expression (kind "featureReference") (reference "ft")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "Btu_IT")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit hour square foot per British thermal unit (IT)"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit hour square foot per British thermal unit (th)"))) (name "degree Fahrenheit hour square foot per British thermal unit (th)") (declared-name "degree Fahrenheit hour square foot per British thermal unit (th)") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "°F")) (expression (kind "featureReference") (reference "h")))) (expression (kind "featureReference") (reference "ft")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "Btu_th")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit hour square foot per British thermal unit (th)"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit second per British thermal unit (IT)"))) (name "degree Fahrenheit second per British thermal unit (IT)") (declared-name "degree Fahrenheit second per British thermal unit (IT)") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "°F")) (expression (kind "featureReference") (reference "s")))) (expression (kind "featureReference") (reference "Btu_IT")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit second per British thermal unit (IT)"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit second per British thermal unit (th)"))) (name "degree Fahrenheit second per British thermal unit (th)") (declared-name "degree Fahrenheit second per British thermal unit (th)") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "°F")) (expression (kind "featureReference") (reference "s")))) (expression (kind "featureReference") (reference "Btu_th")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit second per British thermal unit (th)"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::degree Rankine"))) (name "degree Rankine") (declared-name "degree Rankine") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::degree Rankine::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::degree Rankine")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)"))) (name "degree fahrenheit (absolute temperature scale)") (declared-name "degree fahrenheit (absolute temperature scale)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::definitionalQuantityValues"))) (name "definitionalQuantityValues") (declared-name "definitionalQuantityValues") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::fahrenheitToCelsiusScaleMapping"))) (name "fahrenheitToCelsiusScaleMapping") (declared-name "fahrenheitToCelsiusScaleMapping") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::quantityValueMapping"))) (name "quantityValueMapping") (declared-name "quantityValueMapping") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::temperatureWaterAtFreezingPointInF"))) (name "temperatureWaterAtFreezingPointInF") (declared-name "temperatureWaterAtFreezingPointInF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::unit"))) (name "unit") (declared-name "unit") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::zeroDegreeFahrenheitInKelvin"))) (name "zeroDegreeFahrenheitInKelvin") (declared-name "zeroDegreeFahrenheitInKelvin") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::zeroDegreeFahrenheitToKelvinShift"))) (name "zeroDegreeFahrenheitToKelvinShift") (declared-name "zeroDegreeFahrenheitToKelvinShift") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::fathom (based on US survey foot)"))) (name "fathom (based on US survey foot)") (declared-name "fathom (based on US survey foot)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::fathom (based on US survey foot)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::fathom (based on US survey foot)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::fluid ounce (US)"))) (name "fluid ounce (US)") (declared-name "fluid ounce (US)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::fluid ounce (US)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::fluid ounce (US)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::foot"))) (name "foot") (declared-name "foot") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::foot::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::foot")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::foot (US survey)"))) (name "foot (US survey)") (declared-name "foot (US survey)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::foot (US survey)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::foot (US survey)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::foot of mercury, conventional"))) (name "foot of mercury, conventional") (declared-name "foot of mercury, conventional") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::foot of mercury, conventional::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::foot of mercury, conventional")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::foot of water (39.2 °F)"))) (name "foot of water (39.2 °F)") (declared-name "foot of water (39.2 °F)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::foot of water (39.2 °F)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::foot of water (39.2 °F)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::foot of water, conventional"))) (name "foot of water, conventional") (declared-name "foot of water, conventional") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::foot of water, conventional::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::foot of water, conventional")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::foot per hour"))) (name "foot per hour") (declared-name "foot per hour") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "featureReference") (reference "h")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::foot per hour"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::foot per minute"))) (name "foot per minute") (declared-name "foot per minute") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "featureReference") (reference "min")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::foot per minute"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::foot per second"))) (name "foot per second") (declared-name "foot per second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::foot per second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::foot per second squared"))) (name "foot per second squared") (declared-name "foot per second squared") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "featureReference") (reference "s")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::foot per second squared"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::foot pound-force"))) (name "foot pound-force") (declared-name "foot pound-force") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "featureReference") (reference "lbf")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::foot pound-force"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::foot pound-force per hour"))) (name "foot pound-force per hour") (declared-name "foot pound-force per hour") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "featureReference") (reference "lbf")))) (expression (kind "featureReference") (reference "h")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::foot pound-force per hour"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::foot pound-force per minute"))) (name "foot pound-force per minute") (declared-name "foot pound-force per minute") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "featureReference") (reference "lbf")))) (expression (kind "featureReference") (reference "min")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::foot pound-force per minute"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::foot pound-force per second"))) (name "foot pound-force per second") (declared-name "foot pound-force per second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "featureReference") (reference "lbf")))) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::foot pound-force per second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::foot poundal"))) (name "foot poundal") (declared-name "foot poundal") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::foot poundal::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::foot poundal")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::foot to the fourth power"))) (name "foot to the fourth power") (declared-name "foot to the fourth power") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "integerLiteral") (literal 4)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::foot to the fourth power"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::footcandle"))) (name "footcandle") (declared-name "footcandle") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::footcandle::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::footcandle")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::footlambert"))) (name "footlambert") (declared-name "footlambert") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::footlambert::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::footlambert")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US)"))) (name "gallon (US)") (declared-name "gallon (US)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US) per day"))) (name "gallon (US) per day") (declared-name "gallon (US) per day") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "gal")) (expression (kind "featureReference") (reference "d")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US) per day"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US) per minute (gpm)"))) (name "gallon (US) per minute (gpm)") (declared-name "gallon (US) per minute (gpm)") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "gal")) (expression (kind "featureReference") (reference "min")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US) per minute (gpm)"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::gill (US)"))) (name "gill (US)") (declared-name "gill (US)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::gill (US)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::gill (US)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::grain"))) (name "grain") (declared-name "grain") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::grain::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::grain")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::grain per gallon (US)"))) (name "grain per gallon (US)") (declared-name "grain per gallon (US)") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "gr")) (expression (kind "featureReference") (reference "gal")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::grain per gallon (US)"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (550 ft*lbf/s)"))) (name "horsepower (550 ft*lbf/s)") (declared-name "horsepower (550 ft*lbf/s)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (550 ft*lbf/s)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (550 ft*lbf/s)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (boiler)"))) (name "horsepower (boiler)") (declared-name "horsepower (boiler)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (boiler)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (boiler)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (electric)"))) (name "horsepower (electric)") (declared-name "horsepower (electric)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (electric)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (electric)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (water)"))) (name "horsepower (water)") (declared-name "horsepower (water)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (water)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (water)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (long, 112 lb)"))) (name "hundredweight (long, 112 lb)") (declared-name "hundredweight (long, 112 lb)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (long, 112 lb)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (long, 112 lb)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (short, 100 lb)"))) (name "hundredweight (short, 100 lb)") (declared-name "hundredweight (short, 100 lb)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (short, 100 lb)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (short, 100 lb)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::inch"))) (name "inch") (declared-name "inch") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::inch::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::inch")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (32 °F)"))) (name "inch of mercury (32 °F)") (declared-name "inch of mercury (32 °F)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (32 °F)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (32 °F)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (60 °F)"))) (name "inch of mercury (60 °F)") (declared-name "inch of mercury (60 °F)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (60 °F)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (60 °F)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury, conventional"))) (name "inch of mercury, conventional") (declared-name "inch of mercury, conventional") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury, conventional::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury, conventional")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (39.2 °F)"))) (name "inch of water (39.2 °F)") (declared-name "inch of water (39.2 °F)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (39.2 °F)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (39.2 °F)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (60 °F)"))) (name "inch of water (60 °F)") (declared-name "inch of water (60 °F)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (60 °F)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (60 °F)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of water, conventional"))) (name "inch of water, conventional") (declared-name "inch of water, conventional") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of water, conventional::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::inch of water, conventional")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::inch per second"))) (name "inch per second") (declared-name "inch per second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "in")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::inch per second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::inch per second squared"))) (name "inch per second squared") (declared-name "inch per second squared") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "in")) (expression (kind "featureReference") (reference "s")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::inch per second squared"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::inch to the fourth power"))) (name "inch to the fourth power") (declared-name "inch to the fourth power") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "in")) (expression (kind "integerLiteral") (literal 4)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::inch to the fourth power"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::kip (1 kip = 1000 lbf)"))) (name "kip (1 kip = 1000 lbf)") (declared-name "kip (1 kip = 1000 lbf)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::kip (1 kip = 1000 lbf)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::kip (1 kip = 1000 lbf)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::kip per square inch (ksi)"))) (name "kip per square inch (ksi)") (declared-name "kip per square inch (ksi)") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "kip")) (expression (kind "featureReference") (reference "in")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::kip per square inch (ksi)"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::knot (nautical mile per hour)"))) (name "knot (nautical mile per hour)") (declared-name "knot (nautical mile per hour)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::knot (nautical mile per hour)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::knot (nautical mile per hour)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::lumen per square foot"))) (name "lumen per square foot") (declared-name "lumen per square foot") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lm")) (expression (kind "featureReference") (reference "ft")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::lumen per square foot"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::microinch"))) (name "microinch") (declared-name "microinch") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::microinch::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::microinch")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::mil (0.001 in)"))) (name "mil (0.001 in)") (declared-name "mil (0.001 in)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::mil (0.001 in)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::mil (0.001 in)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::mile"))) (name "mile") (declared-name "mile") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::mile::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::mile")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::mile (based on US survey foot)"))) (name "mile (based on US survey foot)") (declared-name "mile (based on US survey foot)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::mile (based on US survey foot)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::mile (based on US survey foot)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::mile per hour"))) (name "mile per hour") (declared-name "mile per hour") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "mi")) (expression (kind "featureReference") (reference "h")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::mile per hour"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::mile per minute"))) (name "mile per minute") (declared-name "mile per minute") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "mi")) (expression (kind "featureReference") (reference "min")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::mile per minute"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::mile per second"))) (name "mile per second") (declared-name "mile per second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "mi")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::mile per second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::mile, nautical"))) (name "mile, nautical") (declared-name "mile, nautical") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::mile, nautical::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::mile, nautical")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "USCustomaryUnits::mph"))) (name "mph") (declared-name "mph"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::ohm circular-mil per foot"))) (name "ohm circular-mil per foot") (declared-name "ohm circular-mil per foot") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::ohm circular-mil per foot::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::ohm circular-mil per foot")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (US fluid)"))) (name "ounce (US fluid)") (declared-name "ounce (US fluid)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (US fluid)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::ounce (US fluid)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)"))) (name "ounce (avoirdupois)") (declared-name "ounce (avoirdupois)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per cubic inch"))) (name "ounce (avoirdupois) per cubic inch") (declared-name "ounce (avoirdupois) per cubic inch") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "oz")) (expression (kind "featureReference") (reference "in")))) (expression (kind "integerLiteral") (literal 3)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per cubic inch"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per gallon (US)"))) (name "ounce (avoirdupois) per gallon (US)") (declared-name "ounce (avoirdupois) per gallon (US)") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "oz")) (expression (kind "featureReference") (reference "gal")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per gallon (US)"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per square foot"))) (name "ounce (avoirdupois) per square foot") (declared-name "ounce (avoirdupois) per square foot") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "oz")) (expression (kind "featureReference") (reference "ft")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per square foot"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per square inch"))) (name "ounce (avoirdupois) per square inch") (declared-name "ounce (avoirdupois) per square inch") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "oz")) (expression (kind "featureReference") (reference "in")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per square inch"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per square yard"))) (name "ounce (avoirdupois) per square yard") (declared-name "ounce (avoirdupois) per square yard") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "oz")) (expression (kind "featureReference") (reference "yd")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per square yard"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)-force"))) (name "ounce (avoirdupois)-force") (declared-name "ounce (avoirdupois)-force") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)-force::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)-force")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)-force inch"))) (name "ounce (avoirdupois)-force inch") (declared-name "ounce (avoirdupois)-force inch") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "ozf")) (expression (kind "featureReference") (reference "in")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)-force inch"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::peck (US)"))) (name "peck (US)") (declared-name "peck (US)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::peck (US)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::peck (US)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pica (computer) (1/6 in)"))) (name "pica (computer) (1/6 in)") (declared-name "pica (computer) (1/6 in)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::pica (computer) (1/6 in)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::pica (computer) (1/6 in)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pica (printer′s)"))) (name "pica (printer′s)") (declared-name "pica (printer′s)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::pica (printer′s)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::pica (printer′s)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pint (US dry)"))) (name "pint (US dry)") (declared-name "pint (US dry)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::pint (US dry)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::pint (US dry)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pint (US liquid)"))) (name "pint (US liquid)") (declared-name "pint (US liquid)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::pint (US liquid)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::pint (US liquid)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::point (computer) (1/72 in)"))) (name "point (computer) (1/72 in)") (declared-name "point (computer) (1/72 in)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::point (computer) (1/72 in)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::point (computer) (1/72 in)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::point (printer′s)"))) (name "point (printer′s)") (declared-name "point (printer′s)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::point (printer′s)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::point (printer′s)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound (avoirdupois)"))) (name "pound (avoirdupois)") (declared-name "pound (avoirdupois)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound (avoirdupois)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::pound (avoirdupois)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound foot squared"))) (name "pound foot squared") (declared-name "pound foot squared") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "lb")) (expression (kind "featureReference") (reference "ft")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound foot squared"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound inch squared"))) (name "pound inch squared") (declared-name "pound inch squared") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "lb")) (expression (kind "featureReference") (reference "in")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound inch squared"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per cubic foot"))) (name "pound per cubic foot") (declared-name "pound per cubic foot") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lb")) (expression (kind "featureReference") (reference "ft")))) (expression (kind "integerLiteral") (literal 3)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound per cubic foot"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per cubic inch"))) (name "pound per cubic inch") (declared-name "pound per cubic inch") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lb")) (expression (kind "featureReference") (reference "in")))) (expression (kind "integerLiteral") (literal 3)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound per cubic inch"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per cubic yard"))) (name "pound per cubic yard") (declared-name "pound per cubic yard") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lb")) (expression (kind "featureReference") (reference "yd")))) (expression (kind "integerLiteral") (literal 3)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound per cubic yard"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per foot"))) (name "pound per foot") (declared-name "pound per foot") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lb")) (expression (kind "featureReference") (reference "ft")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound per foot"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per foot hour"))) (name "pound per foot hour") (declared-name "pound per foot hour") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lb")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "featureReference") (reference "h")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound per foot hour"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per foot second"))) (name "pound per foot second") (declared-name "pound per foot second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lb")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "featureReference") (reference "s")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound per foot second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per gallon (US)"))) (name "pound per gallon (US)") (declared-name "pound per gallon (US)") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lb")) (expression (kind "featureReference") (reference "gal")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound per gallon (US)"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per hour"))) (name "pound per hour") (declared-name "pound per hour") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lb")) (expression (kind "featureReference") (reference "h")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound per hour"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per inch"))) (name "pound per inch") (declared-name "pound per inch") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lb")) (expression (kind "featureReference") (reference "in")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound per inch"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per minute"))) (name "pound per minute") (declared-name "pound per minute") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lb")) (expression (kind "featureReference") (reference "min")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound per minute"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per second"))) (name "pound per second") (declared-name "pound per second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lb")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound per second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per square foot"))) (name "pound per square foot") (declared-name "pound per square foot") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lb")) (expression (kind "featureReference") (reference "ft")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound per square foot"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per square inch (not pound-force)"))) (name "pound per square inch (not pound-force)") (declared-name "pound per square inch (not pound-force)") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lb")) (expression (kind "featureReference") (reference "in")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound per square inch (not pound-force)"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per yard"))) (name "pound per yard") (declared-name "pound per yard") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lb")) (expression (kind "featureReference") (reference "yd")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound per yard"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force"))) (name "pound-force") (declared-name "pound-force") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::pound-force")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force foot"))) (name "pound-force foot") (declared-name "pound-force foot") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "lbf")) (expression (kind "featureReference") (reference "ft")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound-force foot"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force foot per inch"))) (name "pound-force foot per inch") (declared-name "pound-force foot per inch") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "lbf")) (expression (kind "featureReference") (reference "ft")))) (expression (kind "featureReference") (reference "in")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound-force foot per inch"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force inch"))) (name "pound-force inch") (declared-name "pound-force inch") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "lbf")) (expression (kind "featureReference") (reference "in")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound-force inch"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force inch per inch"))) (name "pound-force inch per inch") (declared-name "pound-force inch per inch") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "lbf")) (expression (kind "featureReference") (reference "in")))) (expression (kind "featureReference") (reference "in")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound-force inch per inch"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per foot"))) (name "pound-force per foot") (declared-name "pound-force per foot") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lbf")) (expression (kind "featureReference") (reference "ft")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per foot"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per inch"))) (name "pound-force per inch") (declared-name "pound-force per inch") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lbf")) (expression (kind "featureReference") (reference "in")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per inch"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per square foot"))) (name "pound-force per square foot") (declared-name "pound-force per square foot") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lbf")) (expression (kind "featureReference") (reference "ft")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per square foot"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per square inch"))) (name "pound-force per square inch") (declared-name "pound-force per square inch") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lbf")) (expression (kind "featureReference") (reference "in")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per square inch"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per square inch (psi)"))) (name "pound-force per square inch (psi)") (declared-name "pound-force per square inch (psi)") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lbf")) (expression (kind "featureReference") (reference "in")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per square inch (psi)"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force second per square foot"))) (name "pound-force second per square foot") (declared-name "pound-force second per square foot") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "lbf")) (expression (kind "featureReference") (reference "s")))) (expression (kind "featureReference") (reference "ft")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound-force second per square foot"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force second per square inch"))) (name "pound-force second per square inch") (declared-name "pound-force second per square inch") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "lbf")) (expression (kind "featureReference") (reference "s")))) (expression (kind "featureReference") (reference "in")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::pound-force second per square inch"))) (role feature-value))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "USCustomaryUnits::psi"))) (name "psi") (declared-name "psi"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::quad (10^15 Btu_IT)"))) (name "quad (10^15 Btu_IT)") (declared-name "quad (10^15 Btu_IT)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::quad (10^15 Btu_IT)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::quad (10^15 Btu_IT)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::quart (US dry)"))) (name "quart (US dry)") (declared-name "quart (US dry)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::quart (US dry)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::quart (US dry)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::quart (US liquid)"))) (name "quart (US liquid)") (declared-name "quart (US liquid)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::quart (US liquid)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::quart (US liquid)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::rod (based on US survey foot)"))) (name "rod (based on US survey foot)") (declared-name "rod (based on US survey foot)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::rod (based on US survey foot)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::rod (based on US survey foot)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::slug"))) (name "slug") (declared-name "slug") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::slug::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::slug")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::slug per cubic foot"))) (name "slug per cubic foot") (declared-name "slug per cubic foot") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "slug")) (expression (kind "featureReference") (reference "ft")))) (expression (kind "integerLiteral") (literal 3)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::slug per cubic foot"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::slug per foot second"))) (name "slug per foot second") (declared-name "slug per foot second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "slug")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "featureReference") (reference "s")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::slug per foot second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::square foot"))) (name "square foot") (declared-name "square foot") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::square foot"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::square foot per hour"))) (name "square foot per hour") (declared-name "square foot per hour") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "h")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::square foot per hour"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::square foot per second"))) (name "square foot per second") (declared-name "square foot per second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "ft")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::square foot per second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::square inch"))) (name "square inch") (declared-name "square inch") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "in")) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::square inch"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::square mile"))) (name "square mile") (declared-name "square mile") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "mi")) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::square mile"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::square mile (based on US survey foot)"))) (name "square mile (based on US survey foot)") (declared-name "square mile (based on US survey foot)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::square mile (based on US survey foot)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::square mile (based on US survey foot)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::square yard"))) (name "square yard") (declared-name "square yard") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "yd")) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::square yard"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::tablespoon"))) (name "tablespoon") (declared-name "tablespoon") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::tablespoon::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::tablespoon")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::teaspoon"))) (name "teaspoon") (declared-name "teaspoon") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::teaspoon::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::teaspoon")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::therm (EC)"))) (name "therm (EC)") (declared-name "therm (EC)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::therm (EC)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::therm (EC)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::therm (US)"))) (name "therm (US)") (declared-name "therm (US)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::therm (US)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::therm (US)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::ton of refrigeration (12 000 Btu_IT/h)"))) (name "ton of refrigeration (12 000 Btu_IT/h)") (declared-name "ton of refrigeration (12 000 Btu_IT/h)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::ton of refrigeration (12 000 Btu_IT/h)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::ton of refrigeration (12 000 Btu_IT/h)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, assay"))) (name "ton, assay") (declared-name "ton, assay") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, assay::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::ton, assay")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, long (2240 lb)"))) (name "ton, long (2240 lb)") (declared-name "ton, long (2240 lb)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, long (2240 lb)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::ton, long (2240 lb)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, long, per cubic yard"))) (name "ton, long, per cubic yard") (declared-name "ton, long, per cubic yard") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, long, per cubic yard::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::ton, long, per cubic yard")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, register"))) (name "ton, register") (declared-name "ton, register") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, register::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::ton, register")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, short (2000 lb)"))) (name "ton, short (2000 lb)") (declared-name "ton, short (2000 lb)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, short (2000 lb)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::ton, short (2000 lb)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per cubic yard"))) (name "ton, short, per cubic yard") (declared-name "ton, short, per cubic yard") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per cubic yard::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per cubic yard")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per hour"))) (name "ton, short, per hour") (declared-name "ton, short, per hour") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per hour::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per hour")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::ton-force (2000 lbf)"))) (name "ton-force (2000 lbf)") (declared-name "ton-force (2000 lbf)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::ton-force (2000 lbf)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::ton-force (2000 lbf)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::unit pole"))) (name "unit pole") (declared-name "unit pole") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::unit pole::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::unit pole")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::watt per square inch"))) (name "watt per square inch") (declared-name "watt per square inch") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "W")) (expression (kind "featureReference") (reference "in")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "USCustomaryUnits::watt per square inch"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "USCustomaryUnits::yard"))) (name "yard") (declared-name "yard") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "USCustomaryUnits::yard::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "USCustomaryUnits::yard")))))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "USCustomaryUnits::_documentation"))) (to (node (document "d0") (qualified-name "USCustomaryUnits"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::_documentation"))) (to (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
