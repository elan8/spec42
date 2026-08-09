# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/SIPrefixes
type=file
~~~
# SOURCE
~~~sysml
standard library package SIPrefixes {
	doc
	/*
	 * Definition of SI unit prefixes as specified in ISO/IEC 80000-1
	 */

	private import MeasurementReferences::*;

	/*
	 * ISO/IEC 80000-1 prefixes for decimal multiples and sub-multiples
	 * 
	 * See also https://en.wikipedia.org/wiki/Unit_prefix
	 */
	attribute yocto: UnitPrefix { :>> longName = "yocto"; :>> symbol = "y"; :>> conversionFactor = 1E-24; }
	attribute zepto: UnitPrefix { :>> longName = "zepto"; :>> symbol = "z"; :>> conversionFactor = 1E-21; }
	attribute atto: UnitPrefix { :>> longName = "atto"; :>> symbol = "a"; :>> conversionFactor = 1E-18; }
	attribute femto: UnitPrefix { :>> longName = "femto"; :>> symbol = "f"; :>> conversionFactor = 1E-15; }
	attribute pico: UnitPrefix { :>> longName = "pico"; :>> symbol = "p"; :>> conversionFactor = 1E-12; }
	attribute nano: UnitPrefix { :>> longName = "nano"; :>> symbol = "n"; :>> conversionFactor = 1E-9; }
	attribute micro: UnitPrefix { :>> longName = "micro"; :>> symbol = "μ"; :>> conversionFactor = 1E-6; }
	attribute milli: UnitPrefix { :>> longName = "milli"; :>> symbol = "m"; :>> conversionFactor = 1E-3; }
	attribute centi: UnitPrefix { :>> longName = "centi"; :>> symbol = "c"; :>> conversionFactor = 1E-2; }
	attribute deci: UnitPrefix { :>> longName = "deci"; :>> symbol = "d"; :>> conversionFactor = 1E-1; }
	attribute deca: UnitPrefix { :>> longName = "deca"; :>> symbol = "da"; :>> conversionFactor = 1E1; }
	attribute hecto: UnitPrefix { :>> longName = "hecto"; :>> symbol = "h"; :>> conversionFactor = 1E2; }
	attribute kilo: UnitPrefix { :>> longName = "kilo"; :>> symbol = "k"; :>> conversionFactor = 1E3; }
	attribute mega: UnitPrefix { :>> longName = "mega"; :>> symbol = "M"; :>> conversionFactor = 1E6; }
	attribute giga: UnitPrefix { :>> longName = "giga"; :>> symbol = "G"; :>> conversionFactor = 1E9; }
	attribute tera: UnitPrefix { :>> longName = "tera"; :>> symbol = "T"; :>> conversionFactor = 1E12; }
	attribute peta: UnitPrefix { :>> longName = "peta"; :>> symbol = "P"; :>> conversionFactor = 1E15; }
	attribute exa: UnitPrefix { :>> longName = "exa"; :>> symbol = "E"; :>> conversionFactor = 1E18; }
	attribute zetta: UnitPrefix { :>> longName = "zetta"; :>> symbol = "Z"; :>> conversionFactor = 1E21; }
	attribute yotta: UnitPrefix { :>> longName = "yotta"; :>> symbol = "Y"; :>> conversionFactor = 1E24; }
	
	/*
	 * ISO/IEC 80000-1 prefixes for binary multiples, i.e. multiples of 1024 (= 2^10)
	 * 
	 * See also https://en.wikipedia.org/wiki/Binary_prefix
	 */
	attribute kibi: UnitPrefix { :>> longName = "kibi"; :>> symbol = "Ki"; :>> conversionFactor = 1024; }
	attribute mebi: UnitPrefix { :>> longName = "mebi"; :>> symbol = "Mi"; :>> conversionFactor = 1024^2; }
	attribute gibi: UnitPrefix { :>> longName = "gibi"; :>> symbol = "Gi"; :>> conversionFactor = 1024^3; }
	attribute tebi: UnitPrefix { :>> longName = "tebi"; :>> symbol = "Ti"; :>> conversionFactor = 1024^4; }
	attribute pebi: UnitPrefix { :>> longName = "pebi"; :>> symbol = "Pi"; :>> conversionFactor = 1024^5; }
	attribute exbi: UnitPrefix { :>> longName = "exbi"; :>> symbol = "Ei"; :>> conversionFactor = 1024^6; }
	attribute zebi: UnitPrefix { :>> longName = "zebi"; :>> symbol = "Zi"; :>> conversionFactor = 1024^7; }
	attribute yobi: UnitPrefix { :>> longName = "yobi"; :>> symbol = "Yi"; :>> conversionFactor = 1024^8; }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'UnitPrefix'
semantic.unresolved_name 'longName'
semantic.unresolved_name 'symbol'
semantic.unresolved_name 'conversionFactor'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,ExponentialValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,ExponentialValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,ExponentialValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,ExponentialValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,ExponentialValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,ExponentialValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,ExponentialValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,ExponentialValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,ExponentialValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,ExponentialValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,ExponentialValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,ExponentialValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,ExponentialValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,ExponentialValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,ExponentialValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,ExponentialValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,ExponentialValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,ExponentialValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,ExponentialValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,ExponentialValue,Semicolon,CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Caret,DecimalValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Caret,DecimalValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Caret,DecimalValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Caret,DecimalValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Caret,DecimalValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Caret,DecimalValue,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Caret,DecimalValue,Semicolon,CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'SIPrefixes'
    (documentation)
    (import_decl private 'MeasurementReferences::*')
    (comment)
    (attribute_usage 'yocto' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'zepto' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'atto' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'femto' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'pico' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'nano' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'micro' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'milli' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'centi' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'deci' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'deca' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'hecto' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'kilo' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'mega' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'giga' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'tera' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'peta' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'exa' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'zetta' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'yotta' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (comment)
    (attribute_usage 'kibi' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'mebi' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'gibi' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'tebi' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'pebi' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'exbi' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'zebi' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))
    (attribute_usage 'yobi' : 'UnitPrefix'
      (default_ref_usage :>> 'longName' value)
      (default_ref_usage :>> 'symbol' value)
      (default_ref_usage :>> 'conversionFactor' value))))
~~~
# FORMAT
~~~sysml
standard library package SIPrefixes {
    doc
    /*
	 * Definition of SI unit prefixes as specified in ISO/IEC 80000-1
	 */

    private import MeasurementReferences::*;

    /*
	 * ISO/IEC 80000-1 prefixes for decimal multiples and sub-multiples
	 * 
	 * See also https://en.wikipedia.org/wiki/Unit_prefix
	 */
    attribute yocto: UnitPrefix { :>> longName = "yocto"; :>> symbol = "y"; :>> conversionFactor = 1E-24; }
    attribute zepto: UnitPrefix { :>> longName = "zepto"; :>> symbol = "z"; :>> conversionFactor = 1E-21; }
    attribute atto: UnitPrefix { :>> longName = "atto"; :>> symbol = "a"; :>> conversionFactor = 1E-18; }
    attribute femto: UnitPrefix { :>> longName = "femto"; :>> symbol = "f"; :>> conversionFactor = 1E-15; }
    attribute pico: UnitPrefix { :>> longName = "pico"; :>> symbol = "p"; :>> conversionFactor = 1E-12; }
    attribute nano: UnitPrefix { :>> longName = "nano"; :>> symbol = "n"; :>> conversionFactor = 1E-9; }
    attribute micro: UnitPrefix { :>> longName = "micro"; :>> symbol = "μ"; :>> conversionFactor = 1E-6; }
    attribute milli: UnitPrefix { :>> longName = "milli"; :>> symbol = "m"; :>> conversionFactor = 1E-3; }
    attribute centi: UnitPrefix { :>> longName = "centi"; :>> symbol = "c"; :>> conversionFactor = 1E-2; }
    attribute deci: UnitPrefix { :>> longName = "deci"; :>> symbol = "d"; :>> conversionFactor = 1E-1; }
    attribute deca: UnitPrefix { :>> longName = "deca"; :>> symbol = "da"; :>> conversionFactor = 1E1; }
    attribute hecto: UnitPrefix { :>> longName = "hecto"; :>> symbol = "h"; :>> conversionFactor = 1E2; }
    attribute kilo: UnitPrefix { :>> longName = "kilo"; :>> symbol = "k"; :>> conversionFactor = 1E3; }
    attribute mega: UnitPrefix { :>> longName = "mega"; :>> symbol = "M"; :>> conversionFactor = 1E6; }
    attribute giga: UnitPrefix { :>> longName = "giga"; :>> symbol = "G"; :>> conversionFactor = 1E9; }
    attribute tera: UnitPrefix { :>> longName = "tera"; :>> symbol = "T"; :>> conversionFactor = 1E12; }
    attribute peta: UnitPrefix { :>> longName = "peta"; :>> symbol = "P"; :>> conversionFactor = 1E15; }
    attribute exa: UnitPrefix { :>> longName = "exa"; :>> symbol = "E"; :>> conversionFactor = 1E18; }
    attribute zetta: UnitPrefix { :>> longName = "zetta"; :>> symbol = "Z"; :>> conversionFactor = 1E21; }
    attribute yotta: UnitPrefix { :>> longName = "yotta"; :>> symbol = "Y"; :>> conversionFactor = 1E24; }

    /*
	 * ISO/IEC 80000-1 prefixes for binary multiples, i.e. multiples of 1024 (= 2^10)
	 * 
	 * See also https://en.wikipedia.org/wiki/Binary_prefix
	 */
    attribute kibi: UnitPrefix { :>> longName = "kibi"; :>> symbol = "Ki"; :>> conversionFactor = 1024; }
    attribute mebi: UnitPrefix { :>> longName = "mebi"; :>> symbol = "Mi"; :>> conversionFactor = 1024^2; }
    attribute gibi: UnitPrefix { :>> longName = "gibi"; :>> symbol = "Gi"; :>> conversionFactor = 1024^3; }
    attribute tebi: UnitPrefix { :>> longName = "tebi"; :>> symbol = "Ti"; :>> conversionFactor = 1024^4; }
    attribute pebi: UnitPrefix { :>> longName = "pebi"; :>> symbol = "Pi"; :>> conversionFactor = 1024^5; }
    attribute exbi: UnitPrefix { :>> longName = "exbi"; :>> symbol = "Ei"; :>> conversionFactor = 1024^6; }
    attribute zebi: UnitPrefix { :>> longName = "zebi"; :>> symbol = "Zi"; :>> conversionFactor = 1024^7; }
    attribute yobi: UnitPrefix { :>> longName = "yobi"; :>> symbol = "Yi"; :>> conversionFactor = 1024^8; }
}

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "SIPrefixes"))) (name "SIPrefixes") (declared-name "SIPrefixes")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "SIPrefixes::*"))) (name "*") (declared-name "*"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "SIPrefixes::_documentation"))) (name ""))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::atto"))) (name "atto") (declared-name "atto") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::atto::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::atto")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::atto::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::atto")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::atto::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::atto")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::centi"))) (name "centi") (declared-name "centi") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::centi::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::centi")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::centi::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::centi")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::centi::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::centi")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::deca"))) (name "deca") (declared-name "deca") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::deca::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::deca")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::deca::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::deca")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::deca::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::deca")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::deci"))) (name "deci") (declared-name "deci") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::deci::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::deci")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::deci::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::deci")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::deci::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::deci")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::exa"))) (name "exa") (declared-name "exa") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::exa::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::exa")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::exa::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::exa")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::exa::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::exa")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::exbi"))) (name "exbi") (declared-name "exbi") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::exbi::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::exbi")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::exbi::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::exbi")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::exbi::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::exbi")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::femto"))) (name "femto") (declared-name "femto") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::femto::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::femto")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::femto::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::femto")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::femto::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::femto")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::gibi"))) (name "gibi") (declared-name "gibi") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::gibi::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::gibi")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::gibi::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::gibi")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::gibi::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::gibi")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::giga"))) (name "giga") (declared-name "giga") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::giga::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::giga")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::giga::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::giga")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::giga::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::giga")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::hecto"))) (name "hecto") (declared-name "hecto") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::hecto::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::hecto")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::hecto::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::hecto")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::hecto::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::hecto")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::kibi"))) (name "kibi") (declared-name "kibi") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::kibi::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::kibi")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::kibi::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::kibi")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::kibi::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::kibi")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::kilo"))) (name "kilo") (declared-name "kilo") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::kilo::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::kilo")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::kilo::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::kilo")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::kilo::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::kilo")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::mebi"))) (name "mebi") (declared-name "mebi") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::mebi::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::mebi")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::mebi::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::mebi")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::mebi::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::mebi")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::mega"))) (name "mega") (declared-name "mega") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::mega::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::mega")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::mega::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::mega")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::mega::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::mega")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::micro"))) (name "micro") (declared-name "micro") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::micro::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::micro")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::micro::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::micro")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::micro::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::micro")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::milli"))) (name "milli") (declared-name "milli") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::milli::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::milli")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::milli::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::milli")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::milli::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::milli")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::nano"))) (name "nano") (declared-name "nano") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::nano::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::nano")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::nano::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::nano")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::nano::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::nano")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::pebi"))) (name "pebi") (declared-name "pebi") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::pebi::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::pebi")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::pebi::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::pebi")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::pebi::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::pebi")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::peta"))) (name "peta") (declared-name "peta") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::peta::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::peta")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::peta::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::peta")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::peta::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::peta")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::pico"))) (name "pico") (declared-name "pico") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::pico::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::pico")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::pico::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::pico")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::pico::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::pico")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::tebi"))) (name "tebi") (declared-name "tebi") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::tebi::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::tebi")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::tebi::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::tebi")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::tebi::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::tebi")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::tera"))) (name "tera") (declared-name "tera") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::tera::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::tera")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::tera::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::tera")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::tera::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::tera")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::yobi"))) (name "yobi") (declared-name "yobi") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::yobi::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::yobi")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::yobi::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::yobi")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::yobi::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::yobi")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::yocto"))) (name "yocto") (declared-name "yocto") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::yocto::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::yocto")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::yocto::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::yocto")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::yocto::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::yocto")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::yotta"))) (name "yotta") (declared-name "yotta") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::yotta::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::yotta")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::yotta::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::yotta")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::yotta::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::yotta")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::zebi"))) (name "zebi") (declared-name "zebi") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::zebi::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::zebi")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::zebi::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::zebi")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::zebi::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::zebi")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::zepto"))) (name "zepto") (declared-name "zepto") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::zepto::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::zepto")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::zepto::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::zepto")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::zepto::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::zepto")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SIPrefixes::zetta"))) (name "zetta") (declared-name "zetta") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::zetta::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::zetta")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::zetta::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::zetta")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SIPrefixes::zetta::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SIPrefixes::zetta")))))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SIPrefixes::_documentation"))) (to (node (document "d0") (qualified-name "SIPrefixes"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
