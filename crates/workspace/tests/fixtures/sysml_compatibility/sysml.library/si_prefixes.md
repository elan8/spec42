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
    doc /*
	 * Definition of SI unit prefixes as specified in ISO/IEC 80000-1
	 */

    private import MeasurementReferences::*;

    /*
	 * ISO/IEC 80000-1 prefixes for decimal multiples and sub-multiples
	 * 
	 * See also https://en.wikipedia.org/wiki/Unit_prefix
	 */
    attribute yocto : UnitPrefix {
        :>> longName = "yocto";
        :>> symbol = "y";
        :>> conversionFactor = 1E-24;
    }
    attribute zepto : UnitPrefix {
        :>> longName = "zepto";
        :>> symbol = "z";
        :>> conversionFactor = 1E-21;
    }
    attribute atto : UnitPrefix {
        :>> longName = "atto";
        :>> symbol = "a";
        :>> conversionFactor = 1E-18;
    }
    attribute femto : UnitPrefix {
        :>> longName = "femto";
        :>> symbol = "f";
        :>> conversionFactor = 1E-15;
    }
    attribute pico : UnitPrefix {
        :>> longName = "pico";
        :>> symbol = "p";
        :>> conversionFactor = 1E-12;
    }
    attribute nano : UnitPrefix {
        :>> longName = "nano";
        :>> symbol = "n";
        :>> conversionFactor = 1E-9;
    }
    attribute micro : UnitPrefix {
        :>> longName = "micro";
        :>> symbol = "μ";
        :>> conversionFactor = 1E-6;
    }
    attribute milli : UnitPrefix {
        :>> longName = "milli";
        :>> symbol = "m";
        :>> conversionFactor = 1E-3;
    }
    attribute centi : UnitPrefix {
        :>> longName = "centi";
        :>> symbol = "c";
        :>> conversionFactor = 1E-2;
    }
    attribute deci : UnitPrefix {
        :>> longName = "deci";
        :>> symbol = "d";
        :>> conversionFactor = 1E-1;
    }
    attribute deca : UnitPrefix {
        :>> longName = "deca";
        :>> symbol = "da";
        :>> conversionFactor = 1E1;
    }
    attribute hecto : UnitPrefix {
        :>> longName = "hecto";
        :>> symbol = "h";
        :>> conversionFactor = 1E2;
    }
    attribute kilo : UnitPrefix {
        :>> longName = "kilo";
        :>> symbol = "k";
        :>> conversionFactor = 1E3;
    }
    attribute mega : UnitPrefix {
        :>> longName = "mega";
        :>> symbol = "M";
        :>> conversionFactor = 1E6;
    }
    attribute giga : UnitPrefix {
        :>> longName = "giga";
        :>> symbol = "G";
        :>> conversionFactor = 1E9;
    }
    attribute tera : UnitPrefix {
        :>> longName = "tera";
        :>> symbol = "T";
        :>> conversionFactor = 1E12;
    }
    attribute peta : UnitPrefix {
        :>> longName = "peta";
        :>> symbol = "P";
        :>> conversionFactor = 1E15;
    }
    attribute exa : UnitPrefix {
        :>> longName = "exa";
        :>> symbol = "E";
        :>> conversionFactor = 1E18;
    }
    attribute zetta : UnitPrefix {
        :>> longName = "zetta";
        :>> symbol = "Z";
        :>> conversionFactor = 1E21;
    }
    attribute yotta : UnitPrefix {
        :>> longName = "yotta";
        :>> symbol = "Y";
        :>> conversionFactor = 1E24;
    }

    /*
	 * ISO/IEC 80000-1 prefixes for binary multiples, i.e. multiples of 1024 (= 2^10)
	 * 
	 * See also https://en.wikipedia.org/wiki/Binary_prefix
	 */
    attribute kibi : UnitPrefix {
        :>> longName = "kibi";
        :>> symbol = "Ki";
        :>> conversionFactor = 1024;
    }
    attribute mebi : UnitPrefix {
        :>> longName = "mebi";
        :>> symbol = "Mi";
        :>> conversionFactor = 1024^2;
    }
    attribute gibi : UnitPrefix {
        :>> longName = "gibi";
        :>> symbol = "Gi";
        :>> conversionFactor = 1024^3;
    }
    attribute tebi : UnitPrefix {
        :>> longName = "tebi";
        :>> symbol = "Ti";
        :>> conversionFactor = 1024^4;
    }
    attribute pebi : UnitPrefix {
        :>> longName = "pebi";
        :>> symbol = "Pi";
        :>> conversionFactor = 1024^5;
    }
    attribute exbi : UnitPrefix {
        :>> longName = "exbi";
        :>> symbol = "Ei";
        :>> conversionFactor = 1024^6;
    }
    attribute zebi : UnitPrefix {
        :>> longName = "zebi";
        :>> symbol = "Zi";
        :>> conversionFactor = 1024^7;
    }
    attribute yobi : UnitPrefix {
        :>> longName = "yobi";
        :>> symbol = "Yi";
        :>> conversionFactor = 1024^8;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'SIPrefixes'
      (documentation)
      (namespace_import private -> 'MeasurementReferences'[unresolved])
      (attribute_usage 'yocto' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'zepto' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'atto' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'femto' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'pico' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'nano' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'micro' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'milli' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'centi' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'deci' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'deca' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'hecto' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'kilo' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'mega' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'giga' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'tera' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'peta' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'exa' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'zetta' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'yotta' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'kibi' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'mebi' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'gibi' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'tebi' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'pebi' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'exbi' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'zebi' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=))))
      (attribute_usage 'yobi' : 'UnitPrefix'[unresolved]
        (reference_usage reference :>> 'longName'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'symbol'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'conversionFactor'[unresolved]
          (feature_value (=)))))))
~~~
