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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "us_customary_units.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 19) (end 8 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 18) (end 9 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 19) (end 10 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 4) (end 12 191))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 60) (end 12 189))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 4) (end 13 198))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 67) (end 13 196))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 4) (end 14 208))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 77) (end 14 206))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 4) (end 15 195))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 66) (end 15 193))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 4) (end 17 195))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 66) (end 17 193))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 4) (end 18 198))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 70) (end 18 196))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 4) (end 19 203))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 75) (end 19 201))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 4) (end 20 203))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 75) (end 20 201))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 4) (end 21 203))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 75) (end 21 201))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 4) (end 22 172))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 4) (end 23 172))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 4) (end 24 174))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 4) (end 25 174))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 4) (end 26 176))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 4) (end 27 176))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 30 4) (end 30 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 4) (end 31 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 4) (end 32 110))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 4) (end 33 110))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 4) (end 34 87))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 4) (end 35 87))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 4) (end 36 165))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 4) (end 37 165))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 4) (end 38 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 4) (end 39 99))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 4) (end 40 99))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 4) (end 41 139))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 42 4) (end 42 139))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 43 4) (end 43 136))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 44 4) (end 44 136))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 45 4) (end 45 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 46 4) (end 46 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 47 4) (end 47 167))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 48 4) (end 48 167))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 51 4) (end 51 131))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 4) (end 52 131))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 4) (end 53 137))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 54 4) (end 54 133))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 4) (end 55 133))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 56 4) (end 56 135))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 57 4) (end 57 179))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 57 48) (end 57 177))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 58 4) (end 58 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 4) (end 59 197))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 68) (end 59 195))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 60 4) (end 60 173))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 60 42) (end 60 171))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 61 4) (end 61 176))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 61 45) (end 61 174))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 62 4) (end 62 178))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 62 47) (end 62 176))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 63 4) (end 63 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 64 4) (end 64 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 65 4) (end 65 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 66 4) (end 66 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 67 4) (end 67 85))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 68 4) (end 68 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 69 4) (end 69 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 70 4) (end 70 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 71 4) (end 71 171))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 71 40) (end 71 169))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 72 4) (end 72 216))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 72 97) (end 72 214))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 73 4) (end 73 129))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 74 4) (end 74 129))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 75 4) (end 75 152))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 76 4) (end 76 152))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 79 4) (end 79 131))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 80 4) (end 80 131))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 81 4) (end 81 504))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 81 4) (end 81 504))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 82 8) (end 82 125))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 87 4) (end 87 193))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 87 64) (end 87 191))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 88 4) (end 88 186))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 88 55) (end 88 184))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 89 4) (end 89 146))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 89 41) (end 89 144))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 90 4) (end 90 177))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 90 48) (end 90 175))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 91 4) (end 91 177))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 91 47) (end 91 175))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 92 4) (end 92 180))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 92 46) (end 92 178))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 93 4) (end 93 200))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 93 70) (end 93 198))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 95 4) (end 95 199))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 95 69) (end 95 197))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 96 4) (end 96 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 97 4) (end 97 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 98 4) (end 98 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 99 4) (end 99 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 100 4) (end 100 173))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 100 44) (end 100 171))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 101 4) (end 101 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 102 4) (end 102 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 103 4) (end 103 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 104 4) (end 104 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 105 4) (end 105 88))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 106 4) (end 106 180))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 106 49) (end 106 178))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 107 4) (end 107 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 109 4) (end 109 88))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 110 4) (end 110 177))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 110 46) (end 110 175))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 111 4) (end 111 149))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 111 40) (end 111 147))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 112 4) (end 112 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 113 4) (end 113 190))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 113 61) (end 113 188))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 114 4) (end 114 178))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 114 50) (end 114 176))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 115 4) (end 115 156))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 115 52) (end 115 154))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 116 4) (end 116 177))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 116 49) (end 116 175))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 117 4) (end 117 188))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 117 58) (end 117 186))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 118 4) (end 118 189))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 118 59) (end 118 187))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 119 4) (end 119 147))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 119 43) (end 119 145))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 120 4) (end 120 187))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 120 58) (end 120 185))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 121 4) (end 121 187))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 121 58) (end 121 185))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 122 4) (end 122 200))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 122 70) (end 122 198))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 124 4) (end 124 184))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 124 56) (end 124 182))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 125 4) (end 125 199))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 125 69) (end 125 197))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 126 4) (end 126 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 127 4) (end 127 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 128 4) (end 128 90))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 129 4) (end 129 188))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 129 59) (end 129 186))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 130 4) (end 130 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 131 4) (end 131 198))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 131 67) (end 131 196))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 133 4) (end 133 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 134 4) (end 134 145))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 134 41) (end 134 143))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 136 4) (end 136 149))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 136 41) (end 136 147))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 137 4) (end 137 191))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 137 62) (end 137 189))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 138 4) (end 138 157))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 138 52) (end 138 155))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 142 4) (end 142 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 144 4) (end 144 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 145 4) (end 145 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 146 4) (end 146 196))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 146 62) (end 146 194))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 147 4) (end 147 184))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 147 54) (end 147 182))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 148 4) (end 148 179))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 148 48) (end 148 177))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 149 4) (end 149 191))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 149 62) (end 149 189))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 150 4) (end 150 91))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 151 4) (end 151 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 152 4) (end 152 90))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 153 4) (end 153 99))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 154 4) (end 154 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 155 4) (end 155 99))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 156 4) (end 156 177))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 156 46) (end 156 175))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 161 4) (end 161 192))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 161 63) (end 161 190))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 162 4) (end 162 179))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 162 50) (end 162 177))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 163 4) (end 163 184))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 163 53) (end 163 182))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 164 4) (end 164 187))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 164 56) (end 164 185))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 165 4) (end 165 192))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 165 63) (end 165 190))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 166 4) (end 166 180))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 166 51) (end 166 178))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 167 4) (end 167 184))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 167 54) (end 167 182))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 168 4) (end 168 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 169 4) (end 169 177))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 169 48) (end 169 175))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 170 4) (end 170 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 171 4) (end 171 84))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 172 4) (end 172 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 173 4) (end 173 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 174 4) (end 174 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 175 4) (end 175 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 177 4) (end 177 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 178 4) (end 178 85))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 180 4) (end 180 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 181 4) (end 181 106))
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
        (range (start 183 4) (end 183 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 184 4) (end 184 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 185 4) (end 185 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 186 4) (end 186 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 187 4) (end 187 87))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 188 4) (end 188 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 189 4) (end 189 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 191 4) (end 191 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 192 4) (end 192 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 193 4) (end 193 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 194 4) (end 194 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 195 4) (end 195 85))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 196 4) (end 196 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 197 4) (end 197 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 198 4) (end 198 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 199 4) (end 199 180))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 199 51) (end 199 178))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 200 4) (end 200 185))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 200 54) (end 200 183))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 201 4) (end 201 188))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 201 57) (end 201 186))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 202 4) (end 202 195))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 202 66) (end 202 193))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 203 4) (end 203 171))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 203 41) (end 203 169))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 204 4) (end 204 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 205 4) (end 205 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 206 4) (end 206 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 207 4) (end 207 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 208 4) (end 208 84))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 209 4) (end 209 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 210 4) (end 210 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 211 4) (end 211 198))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 211 67) (end 211 196))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 212 4) (end 212 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 213 4) (end 213 173))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 213 42) (end 213 171))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 214 4) (end 214 171))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 214 40) (end 214 169))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 215 4) (end 215 149))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 215 42) (end 215 147))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 216 4) (end 216 150))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 216 42) (end 216 148))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 217 4) (end 217 175))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 217 45) (end 217 173))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 218 4) (end 218 180))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 218 51) (end 218 178))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 219 4) (end 219 179))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 219 49) (end 219 177))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 220 4) (end 220 196))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 220 62) (end 220 194))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 221 4) (end 221 198))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 221 69) (end 221 196))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 222 4) (end 222 176))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 222 45) (end 222 174))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 223 4) (end 223 180))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 223 50) (end 223 178))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 224 4) (end 224 197))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 224 63) (end 224 195))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 225 4) (end 225 190))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 225 58) (end 225 188))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 226 4) (end 226 177))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 226 47) (end 226 175))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 227 4) (end 227 87))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 228 4) (end 228 146))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 228 41) (end 228 144))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 231 1) (end 231 1354))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 241 2) (end 241 194))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 245 2) (end 245 223))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 254 8) (end 254 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 254 8) (end 254 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 255 8) (end 255 178))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 255 83) (end 255 97))
      )
    )
  )
)
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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "ba229203349410221384d5c5918f118f577ffc9009cd85bfc58196df788a7034") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits"))) (kind "package") (name "USCustomaryUnits") (declared-name "USCustomaryUnits") (range (start (line 0) (character 0)) (end (line 0) (character 30368))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 8) (character 4)) (end (line 8) (character 44))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 8) (character 19)) (end (line 8) (character 40))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 9) (character 4)) (end (line 9) (character 25))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 9) (character 18)) (end (line 9) (character 21))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 10) (character 4)) (end (line 10) (character 25))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 10) (character 19)) (end (line 10) (character 21))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (39 °F)"))) (kind "attribute def") (name "British thermal unit (39 °F)") (declared-name "British thermal unit (39 °F)") (range (start (line 19) (character 4)) (end (line 19) (character 203))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (39 °F)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 19) (character 75)) (end (line 19) (character 201))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (39 °F)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 19) (character 75)) (end (line 19) (character 93)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (59 °F)"))) (kind "attribute def") (name "British thermal unit (59 °F)") (declared-name "British thermal unit (59 °F)") (range (start (line 20) (character 4)) (end (line 20) (character 203))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (59 °F)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 20) (character 75)) (end (line 20) (character 201))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (59 °F)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 20) (character 75)) (end (line 20) (character 93)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (60 °F)"))) (kind "attribute def") (name "British thermal unit (60 °F)") (declared-name "British thermal unit (60 °F)") (range (start (line 21) (character 4)) (end (line 21) (character 203))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (60 °F)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 21) (character 75)) (end (line 21) (character 201))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (60 °F)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 21) (character 75)) (end (line 21) (character 93)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT)"))) (kind "attribute def") (name "British thermal unit (IT)") (declared-name "British thermal unit (IT)") (range (start (line 15) (character 4)) (end (line 15) (character 195))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) foot per hour square foot degree Fahrenheit"))) (kind "attribute def") (name "British thermal unit (IT) foot per hour square foot degree Fahrenheit") (declared-name "British thermal unit (IT) foot per hour square foot degree Fahrenheit") (range (start (line 22) (character 4)) (end (line 22) (character 172))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalConductivityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) inch per hour square foot degree Fahrenheit"))) (kind "attribute def") (name "British thermal unit (IT) inch per hour square foot degree Fahrenheit") (declared-name "British thermal unit (IT) inch per hour square foot degree Fahrenheit") (range (start (line 24) (character 4)) (end (line 24) (character 174))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalConductivityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) inch per second square foot degree Fahrenheit"))) (kind "attribute def") (name "British thermal unit (IT) inch per second square foot degree Fahrenheit") (declared-name "British thermal unit (IT) inch per second square foot degree Fahrenheit") (range (start (line 26) (character 4)) (end (line 26) (character 176))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalConductivityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per degree Fahrenheit"))) (kind "attribute def") (name "British thermal unit (IT) per degree Fahrenheit") (declared-name "British thermal unit (IT) per degree Fahrenheit") (range (start (line 30) (character 4)) (end (line 30) (character 113))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "HeatCapacityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per degree Rankine"))) (kind "attribute def") (name "British thermal unit (IT) per degree Rankine") (declared-name "British thermal unit (IT) per degree Rankine") (range (start (line 32) (character 4)) (end (line 32) (character 110))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "HeatCapacityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per hour"))) (kind "attribute def") (name "British thermal unit (IT) per hour") (declared-name "British thermal unit (IT) per hour") (range (start (line 34) (character 4)) (end (line 34) (character 87))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per hour square foot degree Fahrenheit"))) (kind "attribute def") (name "British thermal unit (IT) per hour square foot degree Fahrenheit") (declared-name "British thermal unit (IT) per hour square foot degree Fahrenheit") (range (start (line 36) (character 4)) (end (line 36) (character 165))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "CoefficientOfHeatTransferUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per pound"))) (kind "attribute def") (name "British thermal unit (IT) per pound") (declared-name "British thermal unit (IT) per pound") (range (start (line 39) (character 4)) (end (line 39) (character 99))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificEnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per pound degree Fahrenheit"))) (kind "attribute def") (name "British thermal unit (IT) per pound degree Fahrenheit") (declared-name "British thermal unit (IT) per pound degree Fahrenheit") (range (start (line 41) (character 4)) (end (line 41) (character 139))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificHeatCapacityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per pound degree Rankine"))) (kind "attribute def") (name "British thermal unit (IT) per pound degree Rankine") (declared-name "British thermal unit (IT) per pound degree Rankine") (range (start (line 43) (character 4)) (end (line 43) (character 136))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificHeatCapacityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per second"))) (kind "attribute def") (name "British thermal unit (IT) per second") (declared-name "British thermal unit (IT) per second") (range (start (line 45) (character 4)) (end (line 45) (character 89))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per second square foot degree Fahrenheit"))) (kind "attribute def") (name "British thermal unit (IT) per second square foot degree Fahrenheit") (declared-name "British thermal unit (IT) per second square foot degree Fahrenheit") (range (start (line 47) (character 4)) (end (line 47) (character 167))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "CoefficientOfHeatTransferUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per square foot hour"))) (kind "attribute def") (name "British thermal unit (IT) per square foot hour") (declared-name "British thermal unit (IT) per square foot hour") (range (start (line 51) (character 4)) (end (line 51) (character 131))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "DensityOfHeatFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per square foot second"))) (kind "attribute def") (name "British thermal unit (IT) per square foot second") (declared-name "British thermal unit (IT) per square foot second") (range (start (line 54) (character 4)) (end (line 54) (character 133))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "DensityOfHeatFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 15) (character 66)) (end (line 15) (character 193))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 15) (character 66)) (end (line 15) (character 84)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (mean)"))) (kind "attribute def") (name "British thermal unit (mean)") (declared-name "British thermal unit (mean)") (range (start (line 18) (character 4)) (end (line 18) (character 198))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (mean)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 18) (character 70)) (end (line 18) (character 196))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (mean)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 18) (character 70)) (end (line 18) (character 88)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th)"))) (kind "attribute def") (name "British thermal unit (th)") (declared-name "British thermal unit (th)") (range (start (line 17) (character 4)) (end (line 17) (character 195))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) foot per hour square foot degree Fahrenheit"))) (kind "attribute def") (name "British thermal unit (th) foot per hour square foot degree Fahrenheit") (declared-name "British thermal unit (th) foot per hour square foot degree Fahrenheit") (range (start (line 23) (character 4)) (end (line 23) (character 172))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalConductivityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) inch per hour square foot degree Fahrenheit"))) (kind "attribute def") (name "British thermal unit (th) inch per hour square foot degree Fahrenheit") (declared-name "British thermal unit (th) inch per hour square foot degree Fahrenheit") (range (start (line 25) (character 4)) (end (line 25) (character 174))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalConductivityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) inch per second square foot degree Fahrenheit"))) (kind "attribute def") (name "British thermal unit (th) inch per second square foot degree Fahrenheit") (declared-name "British thermal unit (th) inch per second square foot degree Fahrenheit") (range (start (line 27) (character 4)) (end (line 27) (character 176))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalConductivityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per degree Fahrenheit"))) (kind "attribute def") (name "British thermal unit (th) per degree Fahrenheit") (declared-name "British thermal unit (th) per degree Fahrenheit") (range (start (line 31) (character 4)) (end (line 31) (character 113))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "HeatCapacityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per degree Rankine"))) (kind "attribute def") (name "British thermal unit (th) per degree Rankine") (declared-name "British thermal unit (th) per degree Rankine") (range (start (line 33) (character 4)) (end (line 33) (character 110))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "HeatCapacityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per hour"))) (kind "attribute def") (name "British thermal unit (th) per hour") (declared-name "British thermal unit (th) per hour") (range (start (line 35) (character 4)) (end (line 35) (character 87))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per hour square foot degree Fahrenheit"))) (kind "attribute def") (name "British thermal unit (th) per hour square foot degree Fahrenheit") (declared-name "British thermal unit (th) per hour square foot degree Fahrenheit") (range (start (line 37) (character 4)) (end (line 37) (character 165))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "CoefficientOfHeatTransferUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per minute"))) (kind "attribute def") (name "British thermal unit (th) per minute") (declared-name "British thermal unit (th) per minute") (range (start (line 38) (character 4)) (end (line 38) (character 93))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per pound"))) (kind "attribute def") (name "British thermal unit (th) per pound") (declared-name "British thermal unit (th) per pound") (range (start (line 40) (character 4)) (end (line 40) (character 99))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificEnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per pound degree Fahrenheit"))) (kind "attribute def") (name "British thermal unit (th) per pound degree Fahrenheit") (declared-name "British thermal unit (th) per pound degree Fahrenheit") (range (start (line 42) (character 4)) (end (line 42) (character 139))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificHeatCapacityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per pound degree Rankine"))) (kind "attribute def") (name "British thermal unit (th) per pound degree Rankine") (declared-name "British thermal unit (th) per pound degree Rankine") (range (start (line 44) (character 4)) (end (line 44) (character 136))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificHeatCapacityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per second"))) (kind "attribute def") (name "British thermal unit (th) per second") (declared-name "British thermal unit (th) per second") (range (start (line 46) (character 4)) (end (line 46) (character 89))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per second square foot degree Fahrenheit"))) (kind "attribute def") (name "British thermal unit (th) per second square foot degree Fahrenheit") (declared-name "British thermal unit (th) per second square foot degree Fahrenheit") (range (start (line 48) (character 4)) (end (line 48) (character 167))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "CoefficientOfHeatTransferUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per square foot hour"))) (kind "attribute def") (name "British thermal unit (th) per square foot hour") (declared-name "British thermal unit (th) per square foot hour") (range (start (line 52) (character 4)) (end (line 52) (character 131))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "DensityOfHeatFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per square foot minute"))) (kind "attribute def") (name "British thermal unit (th) per square foot minute") (declared-name "British thermal unit (th) per square foot minute") (range (start (line 53) (character 4)) (end (line 53) (character 137))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "DensityOfHeatFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per square foot second"))) (kind "attribute def") (name "British thermal unit (th) per square foot second") (declared-name "British thermal unit (th) per square foot second") (range (start (line 55) (character 4)) (end (line 55) (character 133))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "DensityOfHeatFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per square inch second"))) (kind "attribute def") (name "British thermal unit (th) per square inch second") (declared-name "British thermal unit (th) per square inch second") (range (start (line 56) (character 4)) (end (line 56) (character 135))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "DensityOfHeatFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 17) (character 66)) (end (line 17) (character 193))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 17) (character 66)) (end (line 17) (character 84)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::Btu"))) (kind "alias") (name "Btu") (declared-name "Btu") (range (start (line 16) (character 4)) (end (line 16) (character 25))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::NM"))) (kind "alias") (name "NM") (declared-name "NM") (range (start (line 139) (character 4)) (end (line 139) (character 21))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 30368))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::acre (based on US survey foot)"))) (kind "attribute def") (name "acre (based on US survey foot)") (declared-name "acre (based on US survey foot)") (range (start (line 12) (character 4)) (end (line 12) (character 191))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "AreaUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::acre (based on US survey foot)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 12) (character 60)) (end (line 12) (character 189))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::acre (based on US survey foot)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 12) (character 60)) (end (line 12) (character 78)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::acre foot (based on US survey foot)"))) (kind "attribute def") (name "acre foot (based on US survey foot)") (declared-name "acre foot (based on US survey foot)") (range (start (line 13) (character 4)) (end (line 13) (character 198))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::acre foot (based on US survey foot)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 13) (character 67)) (end (line 13) (character 196))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::acre foot (based on US survey foot)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 13) (character 67)) (end (line 13) (character 85)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::barrel (for petroleum, 42 gallons (US))"))) (kind "attribute def") (name "barrel (for petroleum, 42 gallons (US))") (declared-name "barrel (for petroleum, 42 gallons (US))") (range (start (line 14) (character 4)) (end (line 14) (character 208))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::barrel (for petroleum, 42 gallons (US))::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 14) (character 77)) (end (line 14) (character 206))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::barrel (for petroleum, 42 gallons (US))"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 14) (character 77)) (end (line 14) (character 95)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::bushel (US)"))) (kind "attribute def") (name "bushel (US)") (declared-name "bushel (US)") (range (start (line 57) (character 4)) (end (line 57) (character 179))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::bushel (US)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 57) (character 48)) (end (line 57) (character 177))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::bushel (US)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 57) (character 48)) (end (line 57) (character 66)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::candela per square inch"))) (kind "attribute def") (name "candela per square inch") (declared-name "candela per square inch") (range (start (line 58) (character 4)) (end (line 58) (character 80))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::chain (based on US survey foot)"))) (kind "attribute def") (name "chain (based on US survey foot)") (declared-name "chain (based on US survey foot)") (range (start (line 59) (character 4)) (end (line 59) (character 197))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::chain (based on US survey foot)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 59) (character 68)) (end (line 59) (character 195))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::chain (based on US survey foot)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 59) (character 68)) (end (line 59) (character 86)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::circular mil"))) (kind "attribute def") (name "circular mil") (declared-name "circular mil") (range (start (line 60) (character 4)) (end (line 60) (character 173))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "AreaUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::circular mil::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 60) (character 42)) (end (line 60) (character 171))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::circular mil"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 60) (character 42)) (end (line 60) (character 60)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::clo"))) (kind "attribute def") (name "clo") (declared-name "clo") (range (start (line 61) (character 4)) (end (line 61) (character 176))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalInsulanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::clo::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 61) (character 45)) (end (line 61) (character 174))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::clo"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 61) (character 45)) (end (line 61) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::cord (128 ft^3)"))) (kind "attribute def") (name "cord (128 ft^3)") (declared-name "cord (128 ft^3)") (range (start (line 62) (character 4)) (end (line 62) (character 178))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::cord (128 ft^3)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 62) (character 47)) (end (line 62) (character 176))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::cord (128 ft^3)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 62) (character 47)) (end (line 62) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::cubic foot"))) (kind "attribute def") (name "cubic foot") (declared-name "cubic foot") (range (start (line 63) (character 4)) (end (line 63) (character 56))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::cubic foot per minute"))) (kind "attribute def") (name "cubic foot per minute") (declared-name "cubic foot per minute") (range (start (line 64) (character 4)) (end (line 64) (character 83))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::cubic foot per second"))) (kind "attribute def") (name "cubic foot per second") (declared-name "cubic foot per second") (range (start (line 65) (character 4)) (end (line 65) (character 79))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::cubic inch"))) (kind "attribute def") (name "cubic inch") (declared-name "cubic inch") (range (start (line 66) (character 4)) (end (line 66) (character 58))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::cubic inch per minute"))) (kind "attribute def") (name "cubic inch per minute") (declared-name "cubic inch per minute") (range (start (line 67) (character 4)) (end (line 67) (character 85))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::cubic mile"))) (kind "attribute def") (name "cubic mile") (declared-name "cubic mile") (range (start (line 68) (character 4)) (end (line 68) (character 56))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::cubic yard"))) (kind "attribute def") (name "cubic yard") (declared-name "cubic yard") (range (start (line 69) (character 4)) (end (line 69) (character 56))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::cubic yard per minute"))) (kind "attribute def") (name "cubic yard per minute") (declared-name "cubic yard per minute") (range (start (line 70) (character 4)) (end (line 70) (character 83))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::cup (US)"))) (kind "attribute def") (name "cup (US)") (declared-name "cup (US)") (range (start (line 71) (character 4)) (end (line 71) (character 171))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::cup (US)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 71) (character 40)) (end (line 71) (character 169))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::cup (US)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 71) (character 40)) (end (line 71) (character 58)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit (temperature difference)"))) (kind "attribute def") (name "degree Fahrenheit (temperature difference)") (declared-name "degree Fahrenheit (temperature difference)") (range (start (line 72) (character 4)) (end (line 72) (character 216))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "TemperatureDifferenceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit (temperature difference)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 72) (character 97)) (end (line 72) (character 214))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit (temperature difference)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 72) (character 97)) (end (line 72) (character 115)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit hour per British thermal unit (IT)"))) (kind "attribute def") (name "degree Fahrenheit hour per British thermal unit (IT)") (declared-name "degree Fahrenheit hour per British thermal unit (IT)") (range (start (line 73) (character 4)) (end (line 73) (character 129))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalResistanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit hour per British thermal unit (th)"))) (kind "attribute def") (name "degree Fahrenheit hour per British thermal unit (th)") (declared-name "degree Fahrenheit hour per British thermal unit (th)") (range (start (line 74) (character 4)) (end (line 74) (character 129))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalResistanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit hour square foot per British thermal unit (IT)"))) (kind "attribute def") (name "degree Fahrenheit hour square foot per British thermal unit (IT)") (declared-name "degree Fahrenheit hour square foot per British thermal unit (IT)") (range (start (line 75) (character 4)) (end (line 75) (character 152))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalInsulanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit hour square foot per British thermal unit (th)"))) (kind "attribute def") (name "degree Fahrenheit hour square foot per British thermal unit (th)") (declared-name "degree Fahrenheit hour square foot per British thermal unit (th)") (range (start (line 76) (character 4)) (end (line 76) (character 152))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalInsulanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit second per British thermal unit (IT)"))) (kind "attribute def") (name "degree Fahrenheit second per British thermal unit (IT)") (declared-name "degree Fahrenheit second per British thermal unit (IT)") (range (start (line 79) (character 4)) (end (line 79) (character 131))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalResistanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit second per British thermal unit (th)"))) (kind "attribute def") (name "degree Fahrenheit second per British thermal unit (th)") (declared-name "degree Fahrenheit second per British thermal unit (th)") (range (start (line 80) (character 4)) (end (line 80) (character 131))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalResistanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::degree Rankine"))) (kind "attribute def") (name "degree Rankine") (declared-name "degree Rankine") (range (start (line 81) (character 4)) (end (line 81) (character 504))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermodynamicTemperatureUnit") (range none)) (typing (reference "TemperatureDifferenceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::degree Rankine::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 82) (character 8)) (end (line 82) (character 125))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::degree Rankine"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 82) (character 8)) (end (line 82) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)"))) (kind "attribute def") (name "degree fahrenheit (absolute temperature scale)") (declared-name "degree fahrenheit (absolute temperature scale)") (range (start (line 231) (character 1)) (end (line 231) (character 1354))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "IntervalScale") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::_documentation"))) (kind "documentation") (name "") (range (start (line 231) (character 1)) (end (line 231) (character 1354))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)"))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::definitionalQuantityValues"))) (kind "attribute") (name "definitionalQuantityValues") (declared-name "definitionalQuantityValues") (range (start (line 250) (character 2)) (end (line 250) (character 80))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "definitionalQuantityValues") (range (start (line 250) (character 16)) (end (line 250) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::fahrenheitToCelsiusScaleMapping"))) (kind "attribute") (name "fahrenheitToCelsiusScaleMapping") (declared-name "fahrenheitToCelsiusScaleMapping") (range (start (line 245) (character 2)) (end (line 245) (character 223))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityValueMapping") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::quantityValueMapping"))) (kind "attribute") (name "quantityValueMapping") (declared-name "quantityValueMapping") (range (start (line 251) (character 2)) (end (line 251) (character 71))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityValueMapping") (range (start (line 251) (character 16)) (end (line 251) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::temperatureWaterAtFreezingPointInF"))) (kind "attribute") (name "temperatureWaterAtFreezingPointInF") (declared-name "temperatureWaterAtFreezingPointInF") (range (start (line 241) (character 2)) (end (line 241) (character 194))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "DefinitionalQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::unit"))) (kind "attribute") (name "unit") (declared-name "unit") (range (start (line 240) (character 2)) (end (line 240) (character 19))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "unit") (range (start (line 240) (character 2)) (end (line 240) (character 10)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::zeroDegreeFahrenheitInKelvin"))) (kind "attribute") (name "zeroDegreeFahrenheitInKelvin") (declared-name "zeroDegreeFahrenheitInKelvin") (range (start (line 254) (character 8)) (end (line 254) (character 103))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "ThermodynamicTemperatureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::zeroDegreeFahrenheitToKelvinShift"))) (kind "attribute") (name "zeroDegreeFahrenheitToKelvinShift") (declared-name "zeroDegreeFahrenheitToKelvinShift") (range (start (line 255) (character 8)) (end (line 255) (character 178))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)"))) (authored (membership (kind Feature)) (relationships (typing (reference "CoordinateFramePlacement") (range none)) (redefinition (reference "transformation") (range (start (line 255) (character 83)) (end (line 255) (character 97)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::fathom (based on US survey foot)"))) (kind "attribute def") (name "fathom (based on US survey foot)") (declared-name "fathom (based on US survey foot)") (range (start (line 87) (character 4)) (end (line 87) (character 193))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::fathom (based on US survey foot)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 87) (character 64)) (end (line 87) (character 191))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::fathom (based on US survey foot)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 87) (character 64)) (end (line 87) (character 82)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::fluid ounce (US)"))) (kind "attribute def") (name "fluid ounce (US)") (declared-name "fluid ounce (US)") (range (start (line 88) (character 4)) (end (line 88) (character 186))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::fluid ounce (US)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 88) (character 55)) (end (line 88) (character 184))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::fluid ounce (US)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 88) (character 55)) (end (line 88) (character 73)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::foot"))) (kind "attribute def") (name "foot") (declared-name "foot") (range (start (line 89) (character 4)) (end (line 89) (character 146))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::foot (US survey)"))) (kind "attribute def") (name "foot (US survey)") (declared-name "foot (US survey)") (range (start (line 90) (character 4)) (end (line 90) (character 177))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::foot (US survey)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 90) (character 48)) (end (line 90) (character 175))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::foot (US survey)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 90) (character 48)) (end (line 90) (character 66)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::foot of mercury, conventional"))) (kind "attribute def") (name "foot of mercury, conventional") (declared-name "foot of mercury, conventional") (range (start (line 93) (character 4)) (end (line 93) (character 200))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::foot of mercury, conventional::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 93) (character 70)) (end (line 93) (character 198))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::foot of mercury, conventional"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 93) (character 70)) (end (line 93) (character 88)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::foot of water (39.2 °F)"))) (kind "attribute def") (name "foot of water (39.2 °F)") (declared-name "foot of water (39.2 °F)") (range (start (line 94) (character 4)) (end (line 94) (character 187))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::foot of water (39.2 °F)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 94) (character 58)) (end (line 94) (character 185))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::foot of water (39.2 °F)"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "unitConversion") (range (start (line 94) (character 58)) (end (line 94) (character 76)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::foot of water, conventional"))) (kind "attribute def") (name "foot of water, conventional") (declared-name "foot of water, conventional") (range (start (line 95) (character 4)) (end (line 95) (character 199))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::foot of water, conventional::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 95) (character 69)) (end (line 95) (character 197))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::foot of water, conventional"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 95) (character 69)) (end (line 95) (character 87)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::foot per hour"))) (kind "attribute def") (name "foot per hour") (declared-name "foot per hour") (range (start (line 96) (character 4)) (end (line 96) (character 58))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpeedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::foot per minute"))) (kind "attribute def") (name "foot per minute") (declared-name "foot per minute") (range (start (line 97) (character 4)) (end (line 97) (character 64))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpeedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::foot per second"))) (kind "attribute def") (name "foot per second") (declared-name "foot per second") (range (start (line 98) (character 4)) (end (line 98) (character 60))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpeedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::foot per second squared"))) (kind "attribute def") (name "foot per second squared") (declared-name "foot per second squared") (range (start (line 99) (character 4)) (end (line 99) (character 79))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "AccelerationUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::foot pound-force"))) (kind "attribute def") (name "foot pound-force") (declared-name "foot pound-force") (range (start (line 101) (character 4)) (end (line 101) (character 68))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::foot pound-force per hour"))) (kind "attribute def") (name "foot pound-force per hour") (declared-name "foot pound-force per hour") (range (start (line 102) (character 4)) (end (line 102) (character 80))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::foot pound-force per minute"))) (kind "attribute def") (name "foot pound-force per minute") (declared-name "foot pound-force per minute") (range (start (line 103) (character 4)) (end (line 103) (character 86))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::foot pound-force per second"))) (kind "attribute def") (name "foot pound-force per second") (declared-name "foot pound-force per second") (range (start (line 104) (character 4)) (end (line 104) (character 82))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::foot poundal"))) (kind "attribute def") (name "foot poundal") (declared-name "foot poundal") (range (start (line 100) (character 4)) (end (line 100) (character 173))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::foot poundal::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 100) (character 44)) (end (line 100) (character 171))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::foot poundal"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 100) (character 44)) (end (line 100) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::foot to the fourth power"))) (kind "attribute def") (name "foot to the fourth power") (declared-name "foot to the fourth power") (range (start (line 105) (character 4)) (end (line 105) (character 88))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SecondAxialMomentOfAreaUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::foot::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 89) (character 41)) (end (line 89) (character 144))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::foot"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 89) (character 41)) (end (line 89) (character 59)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::footcandle"))) (kind "attribute def") (name "footcandle") (declared-name "footcandle") (range (start (line 91) (character 4)) (end (line 91) (character 177))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "IlluminanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::footcandle::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 91) (character 47)) (end (line 91) (character 175))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::footcandle"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 91) (character 47)) (end (line 91) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::footlambert"))) (kind "attribute def") (name "footlambert") (declared-name "footlambert") (range (start (line 92) (character 4)) (end (line 92) (character 180))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::footlambert::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 92) (character 46)) (end (line 92) (character 178))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::footlambert"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 92) (character 46)) (end (line 92) (character 64)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US)"))) (kind "attribute def") (name "gallon (US)") (declared-name "gallon (US)") (range (start (line 106) (character 4)) (end (line 106) (character 180))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US) per day"))) (kind "attribute def") (name "gallon (US) per day") (declared-name "gallon (US) per day") (range (start (line 107) (character 4)) (end (line 107) (character 75))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US) per minute (gpm)"))) (kind "attribute def") (name "gallon (US) per minute (gpm)") (declared-name "gallon (US) per minute (gpm)") (range (start (line 109) (character 4)) (end (line 109) (character 88))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 106) (character 49)) (end (line 106) (character 178))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 106) (character 49)) (end (line 106) (character 67)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::gill (US)"))) (kind "attribute def") (name "gill (US)") (declared-name "gill (US)") (range (start (line 110) (character 4)) (end (line 110) (character 177))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::gill (US)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 110) (character 46)) (end (line 110) (character 175))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::gill (US)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 110) (character 46)) (end (line 110) (character 64)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::grain"))) (kind "attribute def") (name "grain") (declared-name "grain") (range (start (line 111) (character 4)) (end (line 111) (character 149))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::grain per gallon (US)"))) (kind "attribute def") (name "grain per gallon (US)") (declared-name "grain per gallon (US)") (range (start (line 112) (character 4)) (end (line 112) (character 76))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::grain::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 111) (character 40)) (end (line 111) (character 147))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::grain"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 111) (character 40)) (end (line 111) (character 58)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (550 ft*lbf/s)"))) (kind "attribute def") (name "horsepower (550 ft*lbf/s)") (declared-name "horsepower (550 ft*lbf/s)") (range (start (line 113) (character 4)) (end (line 113) (character 190))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (550 ft*lbf/s)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 113) (character 61)) (end (line 113) (character 188))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (550 ft*lbf/s)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 113) (character 61)) (end (line 113) (character 79)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (boiler)"))) (kind "attribute def") (name "horsepower (boiler)") (declared-name "horsepower (boiler)") (range (start (line 114) (character 4)) (end (line 114) (character 178))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (boiler)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 114) (character 50)) (end (line 114) (character 176))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (boiler)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 114) (character 50)) (end (line 114) (character 68)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (electric)"))) (kind "attribute def") (name "horsepower (electric)") (declared-name "horsepower (electric)") (range (start (line 115) (character 4)) (end (line 115) (character 156))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (electric)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 115) (character 52)) (end (line 115) (character 154))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (electric)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 115) (character 52)) (end (line 115) (character 70)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (water)"))) (kind "attribute def") (name "horsepower (water)") (declared-name "horsepower (water)") (range (start (line 116) (character 4)) (end (line 116) (character 177))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (water)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 116) (character 49)) (end (line 116) (character 175))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (water)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 116) (character 49)) (end (line 116) (character 67)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (long, 112 lb)"))) (kind "attribute def") (name "hundredweight (long, 112 lb)") (declared-name "hundredweight (long, 112 lb)") (range (start (line 117) (character 4)) (end (line 117) (character 188))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (long, 112 lb)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 117) (character 58)) (end (line 117) (character 186))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (long, 112 lb)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 117) (character 58)) (end (line 117) (character 76)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (short, 100 lb)"))) (kind "attribute def") (name "hundredweight (short, 100 lb)") (declared-name "hundredweight (short, 100 lb)") (range (start (line 118) (character 4)) (end (line 118) (character 189))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (short, 100 lb)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 118) (character 59)) (end (line 118) (character 187))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (short, 100 lb)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 118) (character 59)) (end (line 118) (character 77)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::inch"))) (kind "attribute def") (name "inch") (declared-name "inch") (range (start (line 119) (character 4)) (end (line 119) (character 147))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (32 °F)"))) (kind "attribute def") (name "inch of mercury (32 °F)") (declared-name "inch of mercury (32 °F)") (range (start (line 120) (character 4)) (end (line 120) (character 187))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (32 °F)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 120) (character 58)) (end (line 120) (character 185))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (32 °F)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 120) (character 58)) (end (line 120) (character 76)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (60 °F)"))) (kind "attribute def") (name "inch of mercury (60 °F)") (declared-name "inch of mercury (60 °F)") (range (start (line 121) (character 4)) (end (line 121) (character 187))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (60 °F)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 121) (character 58)) (end (line 121) (character 185))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (60 °F)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 121) (character 58)) (end (line 121) (character 76)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury, conventional"))) (kind "attribute def") (name "inch of mercury, conventional") (declared-name "inch of mercury, conventional") (range (start (line 122) (character 4)) (end (line 122) (character 200))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury, conventional::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 122) (character 70)) (end (line 122) (character 198))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury, conventional"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 122) (character 70)) (end (line 122) (character 88)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (39.2 °F)"))) (kind "attribute def") (name "inch of water (39.2 °F)") (declared-name "inch of water (39.2 °F)") (range (start (line 123) (character 4)) (end (line 123) (character 187))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (39.2 °F)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 123) (character 58)) (end (line 123) (character 185))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (39.2 °F)"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "unitConversion") (range (start (line 123) (character 58)) (end (line 123) (character 76)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (60 °F)"))) (kind "attribute def") (name "inch of water (60 °F)") (declared-name "inch of water (60 °F)") (range (start (line 124) (character 4)) (end (line 124) (character 184))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (60 °F)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 124) (character 56)) (end (line 124) (character 182))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (60 °F)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 124) (character 56)) (end (line 124) (character 74)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of water, conventional"))) (kind "attribute def") (name "inch of water, conventional") (declared-name "inch of water, conventional") (range (start (line 125) (character 4)) (end (line 125) (character 199))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::inch of water, conventional::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 125) (character 69)) (end (line 125) (character 197))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::inch of water, conventional"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 125) (character 69)) (end (line 125) (character 87)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::inch per second"))) (kind "attribute def") (name "inch per second") (declared-name "inch per second") (range (start (line 126) (character 4)) (end (line 126) (character 62))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpeedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::inch per second squared"))) (kind "attribute def") (name "inch per second squared") (declared-name "inch per second squared") (range (start (line 127) (character 4)) (end (line 127) (character 81))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "AccelerationUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::inch to the fourth power"))) (kind "attribute def") (name "inch to the fourth power") (declared-name "inch to the fourth power") (range (start (line 128) (character 4)) (end (line 128) (character 90))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SecondAxialMomentOfAreaUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::inch::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 119) (character 43)) (end (line 119) (character 145))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::inch"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 119) (character 43)) (end (line 119) (character 61)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::kip (1 kip = 1000 lbf)"))) (kind "attribute def") (name "kip (1 kip = 1000 lbf)") (declared-name "kip (1 kip = 1000 lbf)") (range (start (line 129) (character 4)) (end (line 129) (character 188))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "ForceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::kip (1 kip = 1000 lbf)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 129) (character 59)) (end (line 129) (character 186))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::kip (1 kip = 1000 lbf)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 129) (character 59)) (end (line 129) (character 77)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::kip per square inch (ksi)"))) (kind "attribute def") (name "kip per square inch (ksi)") (declared-name "kip per square inch (ksi)") (range (start (line 130) (character 4)) (end (line 130) (character 83))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::knot (nautical mile per hour)"))) (kind "attribute def") (name "knot (nautical mile per hour)") (declared-name "knot (nautical mile per hour)") (range (start (line 131) (character 4)) (end (line 131) (character 198))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpeedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::knot (nautical mile per hour)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 131) (character 67)) (end (line 131) (character 196))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::knot (nautical mile per hour)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 131) (character 67)) (end (line 131) (character 85)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::lumen per square foot"))) (kind "attribute def") (name "lumen per square foot") (declared-name "lumen per square foot") (range (start (line 133) (character 4)) (end (line 133) (character 78))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "IlluminanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::microinch"))) (kind "attribute def") (name "microinch") (declared-name "microinch") (range (start (line 134) (character 4)) (end (line 134) (character 145))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::microinch::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 134) (character 41)) (end (line 134) (character 143))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::microinch"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 134) (character 41)) (end (line 134) (character 59)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::mil (0.001 in)"))) (kind "attribute def") (name "mil (0.001 in)") (declared-name "mil (0.001 in)") (range (start (line 135) (character 4)) (end (line 135) (character 156))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::mil (0.001 in)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 135) (character 52)) (end (line 135) (character 154))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::mil (0.001 in)"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "unitConversion") (range (start (line 135) (character 52)) (end (line 135) (character 70)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::mile"))) (kind "attribute def") (name "mile") (declared-name "mile") (range (start (line 136) (character 4)) (end (line 136) (character 149))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::mile (based on US survey foot)"))) (kind "attribute def") (name "mile (based on US survey foot)") (declared-name "mile (based on US survey foot)") (range (start (line 137) (character 4)) (end (line 137) (character 191))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::mile (based on US survey foot)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 137) (character 62)) (end (line 137) (character 189))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::mile (based on US survey foot)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 137) (character 62)) (end (line 137) (character 80)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::mile per hour"))) (kind "attribute def") (name "mile per hour") (declared-name "mile per hour") (range (start (line 142) (character 4)) (end (line 142) (character 58))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpeedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::mile per minute"))) (kind "attribute def") (name "mile per minute") (declared-name "mile per minute") (range (start (line 144) (character 4)) (end (line 144) (character 64))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpeedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::mile per second"))) (kind "attribute def") (name "mile per second") (declared-name "mile per second") (range (start (line 145) (character 4)) (end (line 145) (character 60))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpeedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::mile, nautical"))) (kind "attribute def") (name "mile, nautical") (declared-name "mile, nautical") (range (start (line 138) (character 4)) (end (line 138) (character 157))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::mile, nautical::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 138) (character 52)) (end (line 138) (character 155))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::mile, nautical"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 138) (character 52)) (end (line 138) (character 70)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::mile::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 136) (character 41)) (end (line 136) (character 147))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::mile"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 136) (character 41)) (end (line 136) (character 59)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::mph"))) (kind "alias") (name "mph") (declared-name "mph") (range (start (line 143) (character 4)) (end (line 143) (character 25))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ohm circular-mil per foot"))) (kind "attribute def") (name "ohm circular-mil per foot") (declared-name "ohm circular-mil per foot") (range (start (line 146) (character 4)) (end (line 146) (character 196))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "ResistivityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ohm circular-mil per foot::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 146) (character 62)) (end (line 146) (character 194))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::ohm circular-mil per foot"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 146) (character 62)) (end (line 146) (character 80)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (US fluid)"))) (kind "attribute def") (name "ounce (US fluid)") (declared-name "ounce (US fluid)") (range (start (line 148) (character 4)) (end (line 148) (character 179))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (US fluid)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 148) (character 48)) (end (line 148) (character 177))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::ounce (US fluid)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 148) (character 48)) (end (line 148) (character 66)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)"))) (kind "attribute def") (name "ounce (avoirdupois)") (declared-name "ounce (avoirdupois)") (range (start (line 147) (character 4)) (end (line 147) (character 184))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per cubic inch"))) (kind "attribute def") (name "ounce (avoirdupois) per cubic inch") (declared-name "ounce (avoirdupois) per cubic inch") (range (start (line 151) (character 4)) (end (line 151) (character 93))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per gallon (US)"))) (kind "attribute def") (name "ounce (avoirdupois) per gallon (US)") (declared-name "ounce (avoirdupois) per gallon (US)") (range (start (line 152) (character 4)) (end (line 152) (character 90))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per square foot"))) (kind "attribute def") (name "ounce (avoirdupois) per square foot") (declared-name "ounce (avoirdupois) per square foot") (range (start (line 153) (character 4)) (end (line 153) (character 99))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SurfaceMassDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per square inch"))) (kind "attribute def") (name "ounce (avoirdupois) per square inch") (declared-name "ounce (avoirdupois) per square inch") (range (start (line 154) (character 4)) (end (line 154) (character 101))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SurfaceMassDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per square yard"))) (kind "attribute def") (name "ounce (avoirdupois) per square yard") (declared-name "ounce (avoirdupois) per square yard") (range (start (line 155) (character 4)) (end (line 155) (character 99))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SurfaceMassDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)-force"))) (kind "attribute def") (name "ounce (avoirdupois)-force") (declared-name "ounce (avoirdupois)-force") (range (start (line 149) (character 4)) (end (line 149) (character 191))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "ForceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)-force inch"))) (kind "attribute def") (name "ounce (avoirdupois)-force inch") (declared-name "ounce (avoirdupois)-force inch") (range (start (line 150) (character 4)) (end (line 150) (character 91))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MomentOfForceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)-force::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 149) (character 62)) (end (line 149) (character 189))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)-force"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 149) (character 62)) (end (line 149) (character 80)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 147) (character 54)) (end (line 147) (character 182))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 147) (character 54)) (end (line 147) (character 72)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::peck (US)"))) (kind "attribute def") (name "peck (US)") (declared-name "peck (US)") (range (start (line 156) (character 4)) (end (line 156) (character 177))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::peck (US)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 156) (character 46)) (end (line 156) (character 175))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::peck (US)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 156) (character 46)) (end (line 156) (character 64)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pica (computer) (1/6 in)"))) (kind "attribute def") (name "pica (computer) (1/6 in)") (declared-name "pica (computer) (1/6 in)") (range (start (line 161) (character 4)) (end (line 161) (character 192))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pica (computer) (1/6 in)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 161) (character 63)) (end (line 161) (character 190))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::pica (computer) (1/6 in)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 161) (character 63)) (end (line 161) (character 81)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pica (printer′s)"))) (kind "attribute def") (name "pica (printer′s)") (declared-name "pica (printer′s)") (range (start (line 162) (character 4)) (end (line 162) (character 179))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pica (printer′s)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 162) (character 50)) (end (line 162) (character 177))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::pica (printer′s)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 162) (character 50)) (end (line 162) (character 68)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pint (US dry)"))) (kind "attribute def") (name "pint (US dry)") (declared-name "pint (US dry)") (range (start (line 163) (character 4)) (end (line 163) (character 184))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pint (US dry)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 163) (character 53)) (end (line 163) (character 182))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::pint (US dry)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 163) (character 53)) (end (line 163) (character 71)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pint (US liquid)"))) (kind "attribute def") (name "pint (US liquid)") (declared-name "pint (US liquid)") (range (start (line 164) (character 4)) (end (line 164) (character 187))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pint (US liquid)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 164) (character 56)) (end (line 164) (character 185))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::pint (US liquid)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 164) (character 56)) (end (line 164) (character 74)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::point (computer) (1/72 in)"))) (kind "attribute def") (name "point (computer) (1/72 in)") (declared-name "point (computer) (1/72 in)") (range (start (line 165) (character 4)) (end (line 165) (character 192))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::point (computer) (1/72 in)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 165) (character 63)) (end (line 165) (character 190))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::point (computer) (1/72 in)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 165) (character 63)) (end (line 165) (character 81)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::point (printer′s)"))) (kind "attribute def") (name "point (printer′s)") (declared-name "point (printer′s)") (range (start (line 166) (character 4)) (end (line 166) (character 180))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::point (printer′s)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 166) (character 51)) (end (line 166) (character 178))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::point (printer′s)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 166) (character 51)) (end (line 166) (character 69)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound (avoirdupois)"))) (kind "attribute def") (name "pound (avoirdupois)") (declared-name "pound (avoirdupois)") (range (start (line 167) (character 4)) (end (line 167) (character 184))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound (avoirdupois)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 167) (character 54)) (end (line 167) (character 182))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::pound (avoirdupois)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 167) (character 54)) (end (line 167) (character 72)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound foot squared"))) (kind "attribute def") (name "pound foot squared") (declared-name "pound foot squared") (range (start (line 168) (character 4)) (end (line 168) (character 81))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MomentOfInertiaUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound inch squared"))) (kind "attribute def") (name "pound inch squared") (declared-name "pound inch squared") (range (start (line 182) (character 4)) (end (line 182) (character 83))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MomentOfInertiaUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per cubic foot"))) (kind "attribute def") (name "pound per cubic foot") (declared-name "pound per cubic foot") (range (start (line 183) (character 4)) (end (line 183) (character 77))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per cubic inch"))) (kind "attribute def") (name "pound per cubic inch") (declared-name "pound per cubic inch") (range (start (line 184) (character 4)) (end (line 184) (character 79))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per cubic yard"))) (kind "attribute def") (name "pound per cubic yard") (declared-name "pound per cubic yard") (range (start (line 185) (character 4)) (end (line 185) (character 77))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per foot"))) (kind "attribute def") (name "pound per foot") (declared-name "pound per foot") (range (start (line 186) (character 4)) (end (line 186) (character 73))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "LinearMassDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per foot hour"))) (kind "attribute def") (name "pound per foot hour") (declared-name "pound per foot hour") (range (start (line 187) (character 4)) (end (line 187) (character 87))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "DynamicViscosityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per foot second"))) (kind "attribute def") (name "pound per foot second") (declared-name "pound per foot second") (range (start (line 188) (character 4)) (end (line 188) (character 89))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "DynamicViscosityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per gallon (US)"))) (kind "attribute def") (name "pound per gallon (US)") (declared-name "pound per gallon (US)") (range (start (line 189) (character 4)) (end (line 189) (character 76))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per hour"))) (kind "attribute def") (name "pound per hour") (declared-name "pound per hour") (range (start (line 191) (character 4)) (end (line 191) (character 66))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per inch"))) (kind "attribute def") (name "pound per inch") (declared-name "pound per inch") (range (start (line 192) (character 4)) (end (line 192) (character 75))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "LinearMassDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per minute"))) (kind "attribute def") (name "pound per minute") (declared-name "pound per minute") (range (start (line 193) (character 4)) (end (line 193) (character 72))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per second"))) (kind "attribute def") (name "pound per second") (declared-name "pound per second") (range (start (line 194) (character 4)) (end (line 194) (character 68))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per square foot"))) (kind "attribute def") (name "pound per square foot") (declared-name "pound per square foot") (range (start (line 195) (character 4)) (end (line 195) (character 85))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SurfaceMassDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per square inch (not pound-force)"))) (kind "attribute def") (name "pound per square inch (not pound-force)") (declared-name "pound per square inch (not pound-force)") (range (start (line 196) (character 4)) (end (line 196) (character 105))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SurfaceMassDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound per yard"))) (kind "attribute def") (name "pound per yard") (declared-name "pound per yard") (range (start (line 197) (character 4)) (end (line 197) (character 73))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "LinearMassDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force"))) (kind "attribute def") (name "pound-force") (declared-name "pound-force") (range (start (line 169) (character 4)) (end (line 169) (character 177))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "ForceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force foot"))) (kind "attribute def") (name "pound-force foot") (declared-name "pound-force foot") (range (start (line 170) (character 4)) (end (line 170) (character 75))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MomentOfForceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force foot per inch"))) (kind "attribute def") (name "pound-force foot per inch") (declared-name "pound-force foot per inch") (range (start (line 171) (character 4)) (end (line 171) (character 84))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "ForceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force inch"))) (kind "attribute def") (name "pound-force inch") (declared-name "pound-force inch") (range (start (line 172) (character 4)) (end (line 172) (character 77))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MomentOfForceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force inch per inch"))) (kind "attribute def") (name "pound-force inch per inch") (declared-name "pound-force inch per inch") (range (start (line 173) (character 4)) (end (line 173) (character 86))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "ForceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per foot"))) (kind "attribute def") (name "pound-force per foot") (declared-name "pound-force per foot") (range (start (line 174) (character 4)) (end (line 174) (character 78))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SurfaceTensionUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per inch"))) (kind "attribute def") (name "pound-force per inch") (declared-name "pound-force per inch") (range (start (line 175) (character 4)) (end (line 175) (character 80))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "SurfaceTensionUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per square foot"))) (kind "attribute def") (name "pound-force per square foot") (declared-name "pound-force per square foot") (range (start (line 177) (character 4)) (end (line 177) (character 83))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per square inch"))) (kind "attribute def") (name "pound-force per square inch") (declared-name "pound-force per square inch") (range (start (line 178) (character 4)) (end (line 178) (character 85))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per square inch (psi)"))) (kind "attribute def") (name "pound-force per square inch (psi)") (declared-name "pound-force per square inch (psi)") (range (start (line 198) (character 4)) (end (line 198) (character 78))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force second per square foot"))) (kind "attribute def") (name "pound-force second per square foot") (declared-name "pound-force second per square foot") (range (start (line 180) (character 4)) (end (line 180) (character 104))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "DynamicViscosityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force second per square inch"))) (kind "attribute def") (name "pound-force second per square inch") (declared-name "pound-force second per square inch") (range (start (line 181) (character 4)) (end (line 181) (character 106))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "DynamicViscosityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::pound-force::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 169) (character 48)) (end (line 169) (character 175))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::pound-force"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 169) (character 48)) (end (line 169) (character 66)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::psi"))) (kind "alias") (name "psi") (declared-name "psi") (range (start (line 179) (character 4)) (end (line 179) (character 29))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::quad (10^15 Btu_IT)"))) (kind "attribute def") (name "quad (10^15 Btu_IT)") (declared-name "quad (10^15 Btu_IT)") (range (start (line 199) (character 4)) (end (line 199) (character 180))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::quad (10^15 Btu_IT)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 199) (character 51)) (end (line 199) (character 178))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::quad (10^15 Btu_IT)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 199) (character 51)) (end (line 199) (character 69)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::quart (US dry)"))) (kind "attribute def") (name "quart (US dry)") (declared-name "quart (US dry)") (range (start (line 200) (character 4)) (end (line 200) (character 185))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::quart (US dry)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 200) (character 54)) (end (line 200) (character 183))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::quart (US dry)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 200) (character 54)) (end (line 200) (character 72)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::quart (US liquid)"))) (kind "attribute def") (name "quart (US liquid)") (declared-name "quart (US liquid)") (range (start (line 201) (character 4)) (end (line 201) (character 188))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::quart (US liquid)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 201) (character 57)) (end (line 201) (character 186))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::quart (US liquid)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 201) (character 57)) (end (line 201) (character 75)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::rod (based on US survey foot)"))) (kind "attribute def") (name "rod (based on US survey foot)") (declared-name "rod (based on US survey foot)") (range (start (line 202) (character 4)) (end (line 202) (character 195))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::rod (based on US survey foot)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 202) (character 66)) (end (line 202) (character 193))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::rod (based on US survey foot)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 202) (character 66)) (end (line 202) (character 84)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::slug"))) (kind "attribute def") (name "slug") (declared-name "slug") (range (start (line 203) (character 4)) (end (line 203) (character 171))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::slug per cubic foot"))) (kind "attribute def") (name "slug per cubic foot") (declared-name "slug per cubic foot") (range (start (line 204) (character 4)) (end (line 204) (character 80))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::slug per foot second"))) (kind "attribute def") (name "slug per foot second") (declared-name "slug per foot second") (range (start (line 205) (character 4)) (end (line 205) (character 92))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "DynamicViscosityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::slug::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 203) (character 41)) (end (line 203) (character 169))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::slug"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 203) (character 41)) (end (line 203) (character 59)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::square foot"))) (kind "attribute def") (name "square foot") (declared-name "square foot") (range (start (line 206) (character 4)) (end (line 206) (character 55))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "AreaUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::square foot per hour"))) (kind "attribute def") (name "square foot per hour") (declared-name "square foot per hour") (range (start (line 207) (character 4)) (end (line 207) (character 82))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "KinematicViscosityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::square foot per second"))) (kind "attribute def") (name "square foot per second") (declared-name "square foot per second") (range (start (line 208) (character 4)) (end (line 208) (character 84))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "KinematicViscosityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::square inch"))) (kind "attribute def") (name "square inch") (declared-name "square inch") (range (start (line 209) (character 4)) (end (line 209) (character 57))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "AreaUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::square mile"))) (kind "attribute def") (name "square mile") (declared-name "square mile") (range (start (line 210) (character 4)) (end (line 210) (character 55))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "AreaUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::square mile (based on US survey foot)"))) (kind "attribute def") (name "square mile (based on US survey foot)") (declared-name "square mile (based on US survey foot)") (range (start (line 211) (character 4)) (end (line 211) (character 198))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "AreaUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::square mile (based on US survey foot)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 211) (character 67)) (end (line 211) (character 196))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::square mile (based on US survey foot)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 211) (character 67)) (end (line 211) (character 85)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::square yard"))) (kind "attribute def") (name "square yard") (declared-name "square yard") (range (start (line 212) (character 4)) (end (line 212) (character 55))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "AreaUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::tablespoon"))) (kind "attribute def") (name "tablespoon") (declared-name "tablespoon") (range (start (line 213) (character 4)) (end (line 213) (character 173))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::tablespoon::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 213) (character 42)) (end (line 213) (character 171))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::tablespoon"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 213) (character 42)) (end (line 213) (character 60)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::teaspoon"))) (kind "attribute def") (name "teaspoon") (declared-name "teaspoon") (range (start (line 214) (character 4)) (end (line 214) (character 171))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::teaspoon::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 214) (character 40)) (end (line 214) (character 169))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::teaspoon"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 214) (character 40)) (end (line 214) (character 58)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::therm (EC)"))) (kind "attribute def") (name "therm (EC)") (declared-name "therm (EC)") (range (start (line 215) (character 4)) (end (line 215) (character 149))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::therm (EC)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 215) (character 42)) (end (line 215) (character 147))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::therm (EC)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 215) (character 42)) (end (line 215) (character 60)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::therm (US)"))) (kind "attribute def") (name "therm (US)") (declared-name "therm (US)") (range (start (line 216) (character 4)) (end (line 216) (character 150))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::therm (US)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 216) (character 42)) (end (line 216) (character 148))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::therm (US)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 216) (character 42)) (end (line 216) (character 60)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ton of refrigeration (12 000 Btu_IT/h)"))) (kind "attribute def") (name "ton of refrigeration (12 000 Btu_IT/h)") (declared-name "ton of refrigeration (12 000 Btu_IT/h)") (range (start (line 221) (character 4)) (end (line 221) (character 198))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ton of refrigeration (12 000 Btu_IT/h)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 221) (character 69)) (end (line 221) (character 196))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::ton of refrigeration (12 000 Btu_IT/h)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 221) (character 69)) (end (line 221) (character 87)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, assay"))) (kind "attribute def") (name "ton, assay") (declared-name "ton, assay") (range (start (line 217) (character 4)) (end (line 217) (character 175))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, assay::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 217) (character 45)) (end (line 217) (character 173))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::ton, assay"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 217) (character 45)) (end (line 217) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, long (2240 lb)"))) (kind "attribute def") (name "ton, long (2240 lb)") (declared-name "ton, long (2240 lb)") (range (start (line 219) (character 4)) (end (line 219) (character 179))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, long (2240 lb)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 219) (character 49)) (end (line 219) (character 177))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::ton, long (2240 lb)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 219) (character 49)) (end (line 219) (character 67)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, long, per cubic yard"))) (kind "attribute def") (name "ton, long, per cubic yard") (declared-name "ton, long, per cubic yard") (range (start (line 220) (character 4)) (end (line 220) (character 196))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, long, per cubic yard::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 220) (character 62)) (end (line 220) (character 194))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::ton, long, per cubic yard"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 220) (character 62)) (end (line 220) (character 80)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, register"))) (kind "attribute def") (name "ton, register") (declared-name "ton, register") (range (start (line 222) (character 4)) (end (line 222) (character 176))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, register::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 222) (character 45)) (end (line 222) (character 174))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::ton, register"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 222) (character 45)) (end (line 222) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, short (2000 lb)"))) (kind "attribute def") (name "ton, short (2000 lb)") (declared-name "ton, short (2000 lb)") (range (start (line 223) (character 4)) (end (line 223) (character 180))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, short (2000 lb)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 223) (character 50)) (end (line 223) (character 178))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::ton, short (2000 lb)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 223) (character 50)) (end (line 223) (character 68)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per cubic yard"))) (kind "attribute def") (name "ton, short, per cubic yard") (declared-name "ton, short, per cubic yard") (range (start (line 224) (character 4)) (end (line 224) (character 197))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per cubic yard::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 224) (character 63)) (end (line 224) (character 195))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per cubic yard"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 224) (character 63)) (end (line 224) (character 81)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per hour"))) (kind "attribute def") (name "ton, short, per hour") (declared-name "ton, short, per hour") (range (start (line 225) (character 4)) (end (line 225) (character 190))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per hour::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 225) (character 58)) (end (line 225) (character 188))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per hour"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 225) (character 58)) (end (line 225) (character 76)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ton-force (2000 lbf)"))) (kind "attribute def") (name "ton-force (2000 lbf)") (declared-name "ton-force (2000 lbf)") (range (start (line 218) (character 4)) (end (line 218) (character 180))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "ForceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::ton-force (2000 lbf)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 218) (character 51)) (end (line 218) (character 178))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::ton-force (2000 lbf)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 218) (character 51)) (end (line 218) (character 69)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::unit pole"))) (kind "attribute def") (name "unit pole") (declared-name "unit pole") (range (start (line 226) (character 4)) (end (line 226) (character 177))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticFluxUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::unit pole::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 226) (character 47)) (end (line 226) (character 175))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::unit pole"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 226) (character 47)) (end (line 226) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::watt per square inch"))) (kind "attribute def") (name "watt per square inch") (declared-name "watt per square inch") (range (start (line 227) (character 4)) (end (line 227) (character 87))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "DensityOfHeatFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::yard"))) (kind "attribute def") (name "yard") (declared-name "yard") (range (start (line 228) (character 4)) (end (line 228) (character 146))) (parent (node (document "d0") (qualified-name "USCustomaryUnits"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "USCustomaryUnits::yard::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 228) (character 41)) (end (line 228) (character 144))) (parent (node (document "d0") (qualified-name "USCustomaryUnits::yard"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 228) (character 41)) (end (line 228) (character 59)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (range (start (line 8) (character 19)) (end (line 8) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 9) (character 18)) (end (line 9) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 10) (character 19)) (end (line 10) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (39 °F)"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (39 °F)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (39 °F)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 19) (character 75)) (end (line 19) (character 93))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (39 °F)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (59 °F)"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (59 °F)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (59 °F)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 20) (character 75)) (end (line 20) (character 93))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (59 °F)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (60 °F)"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (60 °F)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (60 °F)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 21) (character 75)) (end (line 21) (character 93))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (60 °F)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT)"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) foot per hour square foot degree Fahrenheit"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalConductivityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) inch per hour square foot degree Fahrenheit"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalConductivityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) inch per second square foot degree Fahrenheit"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalConductivityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per degree Fahrenheit"))) (kind featureTyping) (ordinal 0)) (authored-target "HeatCapacityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per degree Rankine"))) (kind featureTyping) (ordinal 0)) (authored-target "HeatCapacityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per hour"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per hour square foot degree Fahrenheit"))) (kind featureTyping) (ordinal 0)) (authored-target "CoefficientOfHeatTransferUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per pound"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificEnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per pound degree Fahrenheit"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificHeatCapacityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per pound degree Rankine"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificHeatCapacityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per second"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per second square foot degree Fahrenheit"))) (kind featureTyping) (ordinal 0)) (authored-target "CoefficientOfHeatTransferUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per square foot hour"))) (kind featureTyping) (ordinal 0)) (authored-target "DensityOfHeatFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per square foot second"))) (kind featureTyping) (ordinal 0)) (authored-target "DensityOfHeatFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 15) (character 66)) (end (line 15) (character 84))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (mean)"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (mean)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (mean)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 18) (character 70)) (end (line 18) (character 88))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (mean)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th)"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) foot per hour square foot degree Fahrenheit"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalConductivityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) inch per hour square foot degree Fahrenheit"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalConductivityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) inch per second square foot degree Fahrenheit"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalConductivityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per degree Fahrenheit"))) (kind featureTyping) (ordinal 0)) (authored-target "HeatCapacityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per degree Rankine"))) (kind featureTyping) (ordinal 0)) (authored-target "HeatCapacityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per hour"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per hour square foot degree Fahrenheit"))) (kind featureTyping) (ordinal 0)) (authored-target "CoefficientOfHeatTransferUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per minute"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per pound"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificEnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per pound degree Fahrenheit"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificHeatCapacityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per pound degree Rankine"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificHeatCapacityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per second"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per second square foot degree Fahrenheit"))) (kind featureTyping) (ordinal 0)) (authored-target "CoefficientOfHeatTransferUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per square foot hour"))) (kind featureTyping) (ordinal 0)) (authored-target "DensityOfHeatFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per square foot minute"))) (kind featureTyping) (ordinal 0)) (authored-target "DensityOfHeatFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per square foot second"))) (kind featureTyping) (ordinal 0)) (authored-target "DensityOfHeatFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per square inch second"))) (kind featureTyping) (ordinal 0)) (authored-target "DensityOfHeatFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 17) (character 66)) (end (line 17) (character 84))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::acre (based on US survey foot)"))) (kind featureTyping) (ordinal 0)) (authored-target "AreaUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::acre (based on US survey foot)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::acre (based on US survey foot)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 12) (character 60)) (end (line 12) (character 78))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::acre (based on US survey foot)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::acre foot (based on US survey foot)"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::acre foot (based on US survey foot)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::acre foot (based on US survey foot)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 13) (character 67)) (end (line 13) (character 85))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::acre foot (based on US survey foot)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::barrel (for petroleum, 42 gallons (US))"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::barrel (for petroleum, 42 gallons (US))::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::barrel (for petroleum, 42 gallons (US))::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 14) (character 77)) (end (line 14) (character 95))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::barrel (for petroleum, 42 gallons (US))::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::bushel (US)"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::bushel (US)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::bushel (US)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 57) (character 48)) (end (line 57) (character 66))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::bushel (US)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::candela per square inch"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::chain (based on US survey foot)"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::chain (based on US survey foot)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::chain (based on US survey foot)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 59) (character 68)) (end (line 59) (character 86))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::chain (based on US survey foot)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::circular mil"))) (kind featureTyping) (ordinal 0)) (authored-target "AreaUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::circular mil::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::circular mil::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 60) (character 42)) (end (line 60) (character 60))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::circular mil::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::clo"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalInsulanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::clo::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::clo::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 61) (character 45)) (end (line 61) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::clo::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::cord (128 ft^3)"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::cord (128 ft^3)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::cord (128 ft^3)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 62) (character 47)) (end (line 62) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::cord (128 ft^3)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::cubic foot"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::cubic foot per minute"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::cubic foot per second"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::cubic inch"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::cubic inch per minute"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::cubic mile"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::cubic yard"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::cubic yard per minute"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::cup (US)"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::cup (US)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::cup (US)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 71) (character 40)) (end (line 71) (character 58))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::cup (US)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit (temperature difference)"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureDifferenceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit (temperature difference)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit (temperature difference)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 72) (character 97)) (end (line 72) (character 115))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit (temperature difference)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit hour per British thermal unit (IT)"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalResistanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit hour per British thermal unit (th)"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalResistanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit hour square foot per British thermal unit (IT)"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalInsulanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit hour square foot per British thermal unit (th)"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalInsulanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit second per British thermal unit (IT)"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalResistanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit second per British thermal unit (th)"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalResistanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree Rankine"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermodynamicTemperatureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree Rankine"))) (kind featureTyping) (ordinal 1)) (authored-target "TemperatureDifferenceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree Rankine::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree Rankine::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 82) (character 8)) (end (line 82) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::degree Rankine::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)"))) (kind featureTyping) (ordinal 0)) (authored-target "IntervalScale") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::definitionalQuantityValues"))) (kind redefinition) (ordinal 0)) (authored-target "definitionalQuantityValues") (range (start (line 250) (character 16)) (end (line 250) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::definitionalQuantityValues")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::fahrenheitToCelsiusScaleMapping"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityValueMapping") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::quantityValueMapping"))) (kind redefinition) (ordinal 0)) (authored-target "quantityValueMapping") (range (start (line 251) (character 16)) (end (line 251) (character 36))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::quantityValueMapping")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::temperatureWaterAtFreezingPointInF"))) (kind featureTyping) (ordinal 0)) (authored-target "DefinitionalQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::unit"))) (kind redefinition) (ordinal 0)) (authored-target "unit") (range (start (line 240) (character 2)) (end (line 240) (character 10))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::unit")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::zeroDegreeFahrenheitInKelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermodynamicTemperatureValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::zeroDegreeFahrenheitToKelvinShift"))) (kind featureTyping) (ordinal 0)) (authored-target "CoordinateFramePlacement") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::zeroDegreeFahrenheitToKelvinShift"))) (kind redefinition) (ordinal 0)) (authored-target "transformation") (range (start (line 255) (character 83)) (end (line 255) (character 97))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::fathom (based on US survey foot)"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::fathom (based on US survey foot)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::fathom (based on US survey foot)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 87) (character 64)) (end (line 87) (character 82))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::fathom (based on US survey foot)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::fluid ounce (US)"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::fluid ounce (US)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::fluid ounce (US)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 88) (character 55)) (end (line 88) (character 73))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::fluid ounce (US)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot (US survey)"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot (US survey)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot (US survey)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 90) (character 48)) (end (line 90) (character 66))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::foot (US survey)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot of mercury, conventional"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot of mercury, conventional::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot of mercury, conventional::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 93) (character 70)) (end (line 93) (character 88))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::foot of mercury, conventional::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot of water (39.2 °F)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 94) (character 58)) (end (line 94) (character 76))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::foot of water (39.2 °F)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot of water, conventional"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot of water, conventional::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot of water, conventional::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 95) (character 69)) (end (line 95) (character 87))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::foot of water, conventional::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot per hour"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot per minute"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot per second"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot per second squared"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot pound-force"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot pound-force per hour"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot pound-force per minute"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot pound-force per second"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot poundal"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot poundal::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot poundal::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 100) (character 44)) (end (line 100) (character 62))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::foot poundal::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot to the fourth power"))) (kind featureTyping) (ordinal 0)) (authored-target "SecondAxialMomentOfAreaUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::foot::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 89) (character 41)) (end (line 89) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::foot::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::footcandle"))) (kind featureTyping) (ordinal 0)) (authored-target "IlluminanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::footcandle::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::footcandle::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 91) (character 47)) (end (line 91) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::footcandle::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::footlambert"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::footlambert::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::footlambert::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 92) (character 46)) (end (line 92) (character 64))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::footlambert::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US)"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US) per day"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US) per minute (gpm)"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 106) (character 49)) (end (line 106) (character 67))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::gill (US)"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::gill (US)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::gill (US)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 110) (character 46)) (end (line 110) (character 64))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::gill (US)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::grain"))) (kind featureTyping) (ordinal 0)) (authored-target "MassUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::grain per gallon (US)"))) (kind featureTyping) (ordinal 0)) (authored-target "MassDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::grain::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::grain::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 111) (character 40)) (end (line 111) (character 58))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::grain::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (550 ft*lbf/s)"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (550 ft*lbf/s)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (550 ft*lbf/s)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 113) (character 61)) (end (line 113) (character 79))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (550 ft*lbf/s)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (boiler)"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (boiler)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (boiler)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 114) (character 50)) (end (line 114) (character 68))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (boiler)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (electric)"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (electric)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (electric)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 115) (character 52)) (end (line 115) (character 70))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (electric)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (water)"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (water)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (water)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 116) (character 49)) (end (line 116) (character 67))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (water)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (long, 112 lb)"))) (kind featureTyping) (ordinal 0)) (authored-target "MassUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (long, 112 lb)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (long, 112 lb)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 117) (character 58)) (end (line 117) (character 76))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (long, 112 lb)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (short, 100 lb)"))) (kind featureTyping) (ordinal 0)) (authored-target "MassUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (short, 100 lb)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (short, 100 lb)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 118) (character 59)) (end (line 118) (character 77))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (short, 100 lb)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (32 °F)"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (32 °F)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (32 °F)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 120) (character 58)) (end (line 120) (character 76))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (32 °F)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (60 °F)"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (60 °F)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (60 °F)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 121) (character 58)) (end (line 121) (character 76))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (60 °F)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury, conventional"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury, conventional::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury, conventional::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 122) (character 70)) (end (line 122) (character 88))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury, conventional::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (39.2 °F)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 123) (character 58)) (end (line 123) (character 76))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (39.2 °F)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (60 °F)"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (60 °F)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (60 °F)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 124) (character 56)) (end (line 124) (character 74))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (60 °F)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of water, conventional"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of water, conventional::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of water, conventional::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 125) (character 69)) (end (line 125) (character 87))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::inch of water, conventional::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch per second"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch per second squared"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch to the fourth power"))) (kind featureTyping) (ordinal 0)) (authored-target "SecondAxialMomentOfAreaUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::inch::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 119) (character 43)) (end (line 119) (character 61))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::inch::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::kip (1 kip = 1000 lbf)"))) (kind featureTyping) (ordinal 0)) (authored-target "ForceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::kip (1 kip = 1000 lbf)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::kip (1 kip = 1000 lbf)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 129) (character 59)) (end (line 129) (character 77))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::kip (1 kip = 1000 lbf)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::kip per square inch (ksi)"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::knot (nautical mile per hour)"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::knot (nautical mile per hour)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::knot (nautical mile per hour)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 131) (character 67)) (end (line 131) (character 85))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::knot (nautical mile per hour)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::lumen per square foot"))) (kind featureTyping) (ordinal 0)) (authored-target "IlluminanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::microinch"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::microinch::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::microinch::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 134) (character 41)) (end (line 134) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::microinch::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::mil (0.001 in)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 135) (character 52)) (end (line 135) (character 70))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::mil (0.001 in)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::mile"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::mile (based on US survey foot)"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::mile (based on US survey foot)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::mile (based on US survey foot)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 137) (character 62)) (end (line 137) (character 80))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::mile (based on US survey foot)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::mile per hour"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::mile per minute"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::mile per second"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::mile, nautical"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::mile, nautical::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::mile, nautical::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 138) (character 52)) (end (line 138) (character 70))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::mile, nautical::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::mile::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::mile::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 136) (character 41)) (end (line 136) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::mile::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ohm circular-mil per foot"))) (kind featureTyping) (ordinal 0)) (authored-target "ResistivityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ohm circular-mil per foot::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ohm circular-mil per foot::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 146) (character 62)) (end (line 146) (character 80))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::ohm circular-mil per foot::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ounce (US fluid)"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ounce (US fluid)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ounce (US fluid)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 148) (character 48)) (end (line 148) (character 66))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::ounce (US fluid)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)"))) (kind featureTyping) (ordinal 0)) (authored-target "MassUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per cubic inch"))) (kind featureTyping) (ordinal 0)) (authored-target "MassDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per gallon (US)"))) (kind featureTyping) (ordinal 0)) (authored-target "MassDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per square foot"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceMassDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per square inch"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceMassDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per square yard"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceMassDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)-force"))) (kind featureTyping) (ordinal 0)) (authored-target "ForceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)-force inch"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentOfForceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)-force::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)-force::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 149) (character 62)) (end (line 149) (character 80))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)-force::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 147) (character 54)) (end (line 147) (character 72))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::peck (US)"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::peck (US)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::peck (US)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 156) (character 46)) (end (line 156) (character 64))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::peck (US)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pica (computer) (1/6 in)"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pica (computer) (1/6 in)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pica (computer) (1/6 in)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 161) (character 63)) (end (line 161) (character 81))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::pica (computer) (1/6 in)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pica (printer′s)"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pica (printer′s)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pica (printer′s)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 162) (character 50)) (end (line 162) (character 68))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::pica (printer′s)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pint (US dry)"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pint (US dry)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pint (US dry)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 163) (character 53)) (end (line 163) (character 71))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::pint (US dry)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pint (US liquid)"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pint (US liquid)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pint (US liquid)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 164) (character 56)) (end (line 164) (character 74))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::pint (US liquid)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::point (computer) (1/72 in)"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::point (computer) (1/72 in)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::point (computer) (1/72 in)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 165) (character 63)) (end (line 165) (character 81))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::point (computer) (1/72 in)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::point (printer′s)"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::point (printer′s)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::point (printer′s)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 166) (character 51)) (end (line 166) (character 69))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::point (printer′s)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound (avoirdupois)"))) (kind featureTyping) (ordinal 0)) (authored-target "MassUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound (avoirdupois)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound (avoirdupois)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 167) (character 54)) (end (line 167) (character 72))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::pound (avoirdupois)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound foot squared"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentOfInertiaUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound inch squared"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentOfInertiaUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound per cubic foot"))) (kind featureTyping) (ordinal 0)) (authored-target "MassDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound per cubic inch"))) (kind featureTyping) (ordinal 0)) (authored-target "MassDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound per cubic yard"))) (kind featureTyping) (ordinal 0)) (authored-target "MassDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound per foot"))) (kind featureTyping) (ordinal 0)) (authored-target "LinearMassDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound per foot hour"))) (kind featureTyping) (ordinal 0)) (authored-target "DynamicViscosityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound per foot second"))) (kind featureTyping) (ordinal 0)) (authored-target "DynamicViscosityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound per gallon (US)"))) (kind featureTyping) (ordinal 0)) (authored-target "MassDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound per hour"))) (kind featureTyping) (ordinal 0)) (authored-target "MassFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound per inch"))) (kind featureTyping) (ordinal 0)) (authored-target "LinearMassDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound per minute"))) (kind featureTyping) (ordinal 0)) (authored-target "MassFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound per second"))) (kind featureTyping) (ordinal 0)) (authored-target "MassFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound per square foot"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceMassDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound per square inch (not pound-force)"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceMassDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound per yard"))) (kind featureTyping) (ordinal 0)) (authored-target "LinearMassDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound-force"))) (kind featureTyping) (ordinal 0)) (authored-target "ForceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound-force foot"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentOfForceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound-force foot per inch"))) (kind featureTyping) (ordinal 0)) (authored-target "ForceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound-force inch"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentOfForceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound-force inch per inch"))) (kind featureTyping) (ordinal 0)) (authored-target "ForceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per foot"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceTensionUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per inch"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceTensionUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per square foot"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per square inch"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per square inch (psi)"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound-force second per square foot"))) (kind featureTyping) (ordinal 0)) (authored-target "DynamicViscosityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound-force second per square inch"))) (kind featureTyping) (ordinal 0)) (authored-target "DynamicViscosityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound-force::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::pound-force::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 169) (character 48)) (end (line 169) (character 66))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::pound-force::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::quad (10^15 Btu_IT)"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::quad (10^15 Btu_IT)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::quad (10^15 Btu_IT)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 199) (character 51)) (end (line 199) (character 69))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::quad (10^15 Btu_IT)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::quart (US dry)"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::quart (US dry)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::quart (US dry)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 200) (character 54)) (end (line 200) (character 72))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::quart (US dry)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::quart (US liquid)"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::quart (US liquid)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::quart (US liquid)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 201) (character 57)) (end (line 201) (character 75))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::quart (US liquid)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::rod (based on US survey foot)"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::rod (based on US survey foot)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::rod (based on US survey foot)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 202) (character 66)) (end (line 202) (character 84))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::rod (based on US survey foot)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::slug"))) (kind featureTyping) (ordinal 0)) (authored-target "MassUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::slug per cubic foot"))) (kind featureTyping) (ordinal 0)) (authored-target "MassDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::slug per foot second"))) (kind featureTyping) (ordinal 0)) (authored-target "DynamicViscosityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::slug::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::slug::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 203) (character 41)) (end (line 203) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::slug::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::square foot"))) (kind featureTyping) (ordinal 0)) (authored-target "AreaUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::square foot per hour"))) (kind featureTyping) (ordinal 0)) (authored-target "KinematicViscosityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::square foot per second"))) (kind featureTyping) (ordinal 0)) (authored-target "KinematicViscosityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::square inch"))) (kind featureTyping) (ordinal 0)) (authored-target "AreaUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::square mile"))) (kind featureTyping) (ordinal 0)) (authored-target "AreaUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::square mile (based on US survey foot)"))) (kind featureTyping) (ordinal 0)) (authored-target "AreaUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::square mile (based on US survey foot)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::square mile (based on US survey foot)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 211) (character 67)) (end (line 211) (character 85))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::square mile (based on US survey foot)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::square yard"))) (kind featureTyping) (ordinal 0)) (authored-target "AreaUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::tablespoon"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::tablespoon::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::tablespoon::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 213) (character 42)) (end (line 213) (character 60))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::tablespoon::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::teaspoon"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::teaspoon::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::teaspoon::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 214) (character 40)) (end (line 214) (character 58))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::teaspoon::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::therm (EC)"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::therm (EC)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::therm (EC)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 215) (character 42)) (end (line 215) (character 60))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::therm (EC)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::therm (US)"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::therm (US)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::therm (US)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 216) (character 42)) (end (line 216) (character 60))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::therm (US)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton of refrigeration (12 000 Btu_IT/h)"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton of refrigeration (12 000 Btu_IT/h)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton of refrigeration (12 000 Btu_IT/h)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 221) (character 69)) (end (line 221) (character 87))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::ton of refrigeration (12 000 Btu_IT/h)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, assay"))) (kind featureTyping) (ordinal 0)) (authored-target "MassUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, assay::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, assay::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 217) (character 45)) (end (line 217) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::ton, assay::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, long (2240 lb)"))) (kind featureTyping) (ordinal 0)) (authored-target "MassUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, long (2240 lb)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, long (2240 lb)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 219) (character 49)) (end (line 219) (character 67))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::ton, long (2240 lb)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, long, per cubic yard"))) (kind featureTyping) (ordinal 0)) (authored-target "MassDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, long, per cubic yard::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, long, per cubic yard::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 220) (character 62)) (end (line 220) (character 80))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::ton, long, per cubic yard::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, register"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, register::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, register::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 222) (character 45)) (end (line 222) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::ton, register::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, short (2000 lb)"))) (kind featureTyping) (ordinal 0)) (authored-target "MassUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, short (2000 lb)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, short (2000 lb)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 223) (character 50)) (end (line 223) (character 68))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::ton, short (2000 lb)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per cubic yard"))) (kind featureTyping) (ordinal 0)) (authored-target "MassDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per cubic yard::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per cubic yard::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 224) (character 63)) (end (line 224) (character 81))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per cubic yard::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per hour"))) (kind featureTyping) (ordinal 0)) (authored-target "MassFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per hour::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per hour::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 225) (character 58)) (end (line 225) (character 76))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per hour::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton-force (2000 lbf)"))) (kind featureTyping) (ordinal 0)) (authored-target "ForceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton-force (2000 lbf)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::ton-force (2000 lbf)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 218) (character 51)) (end (line 218) (character 69))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::ton-force (2000 lbf)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::unit pole"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticFluxUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::unit pole::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::unit pole::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 226) (character 47)) (end (line 226) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::unit pole::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::watt per square inch"))) (kind featureTyping) (ordinal 0)) (authored-target "DensityOfHeatFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::yard"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::yard::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "USCustomaryUnits::yard::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 228) (character 41)) (end (line 228) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "USCustomaryUnits::yard::unitConversion")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (39 °F)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (39 °F)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (39 °F)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (59 °F)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (59 °F)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (59 °F)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (60 °F)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (60 °F)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (60 °F)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (mean)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (mean)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (mean)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::acre (based on US survey foot)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::acre (based on US survey foot)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::acre (based on US survey foot)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::acre foot (based on US survey foot)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::acre foot (based on US survey foot)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::acre foot (based on US survey foot)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::barrel (for petroleum, 42 gallons (US))::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::barrel (for petroleum, 42 gallons (US))::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::barrel (for petroleum, 42 gallons (US))::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::bushel (US)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::bushel (US)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::bushel (US)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::chain (based on US survey foot)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::chain (based on US survey foot)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::chain (based on US survey foot)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::circular mil::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::circular mil::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::circular mil::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::clo::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::clo::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::clo::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::cord (128 ft^3)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::cord (128 ft^3)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::cord (128 ft^3)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::cup (US)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::cup (US)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::cup (US)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit (temperature difference)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit (temperature difference)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit (temperature difference)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::degree Rankine::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::degree Rankine::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::degree Rankine::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::definitionalQuantityValues"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::definitionalQuantityValues"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::definitionalQuantityValues"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::quantityValueMapping"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::quantityValueMapping"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::quantityValueMapping"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::unit"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::unit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::degree fahrenheit (absolute temperature scale)::unit"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::fathom (based on US survey foot)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::fathom (based on US survey foot)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::fathom (based on US survey foot)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::fluid ounce (US)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::fluid ounce (US)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::fluid ounce (US)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::foot (US survey)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::foot (US survey)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::foot (US survey)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::foot of mercury, conventional::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::foot of mercury, conventional::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::foot of mercury, conventional::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::foot of water (39.2 °F)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::foot of water (39.2 °F)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::foot of water (39.2 °F)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::foot of water, conventional::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::foot of water, conventional::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::foot of water, conventional::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::foot poundal::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::foot poundal::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::foot poundal::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::foot::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::foot::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::foot::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::footcandle::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::footcandle::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::footcandle::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::footlambert::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::footlambert::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::footlambert::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::gill (US)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::gill (US)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::gill (US)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::grain::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::grain::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::grain::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (550 ft*lbf/s)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (550 ft*lbf/s)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (550 ft*lbf/s)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (boiler)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (boiler)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (boiler)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (electric)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (electric)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (electric)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (water)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (water)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::horsepower (water)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (long, 112 lb)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (long, 112 lb)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (long, 112 lb)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (short, 100 lb)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (short, 100 lb)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::hundredweight (short, 100 lb)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (32 °F)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (32 °F)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (32 °F)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (60 °F)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (60 °F)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury (60 °F)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury, conventional::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury, conventional::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of mercury, conventional::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (39.2 °F)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (39.2 °F)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (39.2 °F)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (60 °F)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (60 °F)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of water (60 °F)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of water, conventional::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::inch of water, conventional::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::inch of water, conventional::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::inch::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::inch::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::inch::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::kip (1 kip = 1000 lbf)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::kip (1 kip = 1000 lbf)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::kip (1 kip = 1000 lbf)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::knot (nautical mile per hour)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::knot (nautical mile per hour)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::knot (nautical mile per hour)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::microinch::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::microinch::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::microinch::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::mil (0.001 in)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::mil (0.001 in)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::mil (0.001 in)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::mile (based on US survey foot)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::mile (based on US survey foot)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::mile (based on US survey foot)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::mile, nautical::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::mile, nautical::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::mile, nautical::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::mile::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::mile::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::mile::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::ohm circular-mil per foot::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::ohm circular-mil per foot::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::ohm circular-mil per foot::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::ounce (US fluid)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::ounce (US fluid)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::ounce (US fluid)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)-force::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)-force::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)-force::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::peck (US)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::peck (US)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::peck (US)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::pica (computer) (1/6 in)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::pica (computer) (1/6 in)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::pica (computer) (1/6 in)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::pica (printer′s)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::pica (printer′s)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::pica (printer′s)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::pint (US dry)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::pint (US dry)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::pint (US dry)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::pint (US liquid)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::pint (US liquid)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::pint (US liquid)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::point (computer) (1/72 in)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::point (computer) (1/72 in)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::point (computer) (1/72 in)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::point (printer′s)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::point (printer′s)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::point (printer′s)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::pound (avoirdupois)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::pound (avoirdupois)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::pound (avoirdupois)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::pound-force::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::pound-force::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::pound-force::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::quad (10^15 Btu_IT)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::quad (10^15 Btu_IT)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::quad (10^15 Btu_IT)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::quart (US dry)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::quart (US dry)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::quart (US dry)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::quart (US liquid)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::quart (US liquid)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::quart (US liquid)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::rod (based on US survey foot)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::rod (based on US survey foot)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::rod (based on US survey foot)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::slug::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::slug::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::slug::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::square mile (based on US survey foot)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::square mile (based on US survey foot)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::square mile (based on US survey foot)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::tablespoon::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::tablespoon::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::tablespoon::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::teaspoon::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::teaspoon::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::teaspoon::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::therm (EC)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::therm (EC)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::therm (EC)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::therm (US)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::therm (US)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::therm (US)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::ton of refrigeration (12 000 Btu_IT/h)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::ton of refrigeration (12 000 Btu_IT/h)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::ton of refrigeration (12 000 Btu_IT/h)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, assay::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::ton, assay::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, assay::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, long (2240 lb)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::ton, long (2240 lb)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, long (2240 lb)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, long, per cubic yard::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::ton, long, per cubic yard::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, long, per cubic yard::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, register::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::ton, register::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, register::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, short (2000 lb)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::ton, short (2000 lb)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, short (2000 lb)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per cubic yard::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per cubic yard::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per cubic yard::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per hour::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per hour::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::ton, short, per hour::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::ton-force (2000 lbf)::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::ton-force (2000 lbf)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::ton-force (2000 lbf)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::unit pole::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::unit pole::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::unit pole::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "USCustomaryUnits::yard::unitConversion"))) (target (node (document "d0") (qualified-name "USCustomaryUnits::yard::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "USCustomaryUnits::yard::unitConversion"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) foot per hour square foot degree Fahrenheit")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) inch per hour square foot degree Fahrenheit")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) inch per second square foot degree Fahrenheit")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per degree Fahrenheit")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per degree Rankine")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per hour")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per hour square foot degree Fahrenheit")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per pound")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per pound degree Fahrenheit")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per pound degree Rankine")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per second square foot degree Fahrenheit")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per square foot hour")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (IT) per square foot second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) foot per hour square foot degree Fahrenheit")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) inch per hour square foot degree Fahrenheit")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) inch per second square foot degree Fahrenheit")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per degree Fahrenheit")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per degree Rankine")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per hour")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per hour square foot degree Fahrenheit")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per minute")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per pound")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per pound degree Fahrenheit")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per pound degree Rankine")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per second square foot degree Fahrenheit")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per square foot hour")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per square foot minute")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per square foot second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::British thermal unit (th) per square inch second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::candela per square inch")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::cubic foot")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::cubic foot per minute")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::cubic foot per second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::cubic inch")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::cubic inch per minute")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::cubic mile")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::cubic yard")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::cubic yard per minute")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit hour per British thermal unit (IT)")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit hour per British thermal unit (th)")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit hour square foot per British thermal unit (IT)")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit hour square foot per British thermal unit (th)")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit second per British thermal unit (IT)")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::degree Fahrenheit second per British thermal unit (th)")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::foot per hour")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::foot per minute")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::foot per second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::foot per second squared")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::foot pound-force")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::foot pound-force per hour")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::foot pound-force per minute")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::foot pound-force per second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::foot to the fourth power")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US) per day")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::gallon (US) per minute (gpm)")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::grain per gallon (US)")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::inch per second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::inch per second squared")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::inch to the fourth power")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::kip per square inch (ksi)")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::lumen per square foot")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::mile per hour")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::mile per minute")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::mile per second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per cubic inch")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per gallon (US)")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per square foot")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per square inch")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois) per square yard")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::ounce (avoirdupois)-force inch")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound foot squared")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound inch squared")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound per cubic foot")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound per cubic inch")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound per cubic yard")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound per foot")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound per foot hour")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound per foot second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound per gallon (US)")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound per hour")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound per inch")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound per minute")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound per second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound per square foot")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound per square inch (not pound-force)")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound per yard")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound-force foot")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound-force foot per inch")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound-force inch")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound-force inch per inch")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per foot")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per inch")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per square foot")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per square inch")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound-force per square inch (psi)")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound-force second per square foot")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::pound-force second per square inch")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::slug per cubic foot")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::slug per foot second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::square foot")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::square foot per hour")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::square foot per second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::square inch")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::square mile")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::square yard")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "USCustomaryUnits::watt per square inch")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
