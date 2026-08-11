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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "si_prefixes.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 1) (end 13 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 1) (end 14 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 1) (end 15 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 1) (end 16 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 1) (end 17 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 1) (end 18 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 1) (end 19 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 1) (end 20 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 1) (end 21 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 1) (end 22 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 1) (end 23 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 1) (end 24 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 1) (end 25 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 1) (end 26 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 1) (end 27 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 1) (end 28 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 1) (end 29 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 30 1) (end 30 99))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 1) (end 31 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 1) (end 32 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 1) (end 39 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 1) (end 40 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 1) (end 41 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 42 1) (end 42 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 43 1) (end 43 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 44 1) (end 44 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 45 1) (end 45 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 46 1) (end 46 104))
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "4b77b31602e9ce9ad7200250f2d72bbf8eb44a01ddf42be6559935b2136cf628") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "SIPrefixes"))) (kind "package") (name "SIPrefixes") (declared-name "SIPrefixes") (range (start (line 0) (character 0)) (end (line 0) (character 3354))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 6) (character 1)) (end (line 6) (character 41))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 37))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 3354))) (parent (node (document "d0") (qualified-name "SIPrefixes"))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::atto"))) (kind "attribute def") (name "atto") (declared-name "atto") (range (start (line 15) (character 1)) (end (line 15) (character 102))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::atto::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 15) (character 71)) (end (line 15) (character 100))) (parent (node (document "d0") (qualified-name "SIPrefixes::atto"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 15) (character 71)) (end (line 15) (character 91)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::atto::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 15) (character 30)) (end (line 15) (character 52))) (parent (node (document "d0") (qualified-name "SIPrefixes::atto"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 15) (character 30)) (end (line 15) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::atto::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 15) (character 53)) (end (line 15) (character 70))) (parent (node (document "d0") (qualified-name "SIPrefixes::atto"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 15) (character 53)) (end (line 15) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::centi"))) (kind "attribute def") (name "centi") (declared-name "centi") (range (start (line 21) (character 1)) (end (line 21) (character 103))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::centi::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 21) (character 73)) (end (line 21) (character 101))) (parent (node (document "d0") (qualified-name "SIPrefixes::centi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 21) (character 73)) (end (line 21) (character 93)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::centi::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 21) (character 31)) (end (line 21) (character 54))) (parent (node (document "d0") (qualified-name "SIPrefixes::centi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 21) (character 31)) (end (line 21) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::centi::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 21) (character 55)) (end (line 21) (character 72))) (parent (node (document "d0") (qualified-name "SIPrefixes::centi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 21) (character 55)) (end (line 21) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::deca"))) (kind "attribute def") (name "deca") (declared-name "deca") (range (start (line 23) (character 1)) (end (line 23) (character 101))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::deca::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 23) (character 72)) (end (line 23) (character 99))) (parent (node (document "d0") (qualified-name "SIPrefixes::deca"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 23) (character 72)) (end (line 23) (character 92)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::deca::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 23) (character 30)) (end (line 23) (character 52))) (parent (node (document "d0") (qualified-name "SIPrefixes::deca"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 23) (character 30)) (end (line 23) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::deca::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 23) (character 53)) (end (line 23) (character 71))) (parent (node (document "d0") (qualified-name "SIPrefixes::deca"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 23) (character 53)) (end (line 23) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::deci"))) (kind "attribute def") (name "deci") (declared-name "deci") (range (start (line 22) (character 1)) (end (line 22) (character 101))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::deci::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 22) (character 71)) (end (line 22) (character 99))) (parent (node (document "d0") (qualified-name "SIPrefixes::deci"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 22) (character 71)) (end (line 22) (character 91)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::deci::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 22) (character 30)) (end (line 22) (character 52))) (parent (node (document "d0") (qualified-name "SIPrefixes::deci"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 22) (character 30)) (end (line 22) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::deci::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 22) (character 53)) (end (line 22) (character 70))) (parent (node (document "d0") (qualified-name "SIPrefixes::deci"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 22) (character 53)) (end (line 22) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::exa"))) (kind "attribute def") (name "exa") (declared-name "exa") (range (start (line 30) (character 1)) (end (line 30) (character 99))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::exa::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 30) (character 69)) (end (line 30) (character 97))) (parent (node (document "d0") (qualified-name "SIPrefixes::exa"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 30) (character 69)) (end (line 30) (character 89)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::exa::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 30) (character 29)) (end (line 30) (character 50))) (parent (node (document "d0") (qualified-name "SIPrefixes::exa"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 30) (character 29)) (end (line 30) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::exa::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 30) (character 51)) (end (line 30) (character 68))) (parent (node (document "d0") (qualified-name "SIPrefixes::exa"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 30) (character 51)) (end (line 30) (character 61)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::exbi"))) (kind "attribute def") (name "exbi") (declared-name "exbi") (range (start (line 44) (character 1)) (end (line 44) (character 104))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::exbi::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 44) (character 72)) (end (line 44) (character 102))) (parent (node (document "d0") (qualified-name "SIPrefixes::exbi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 44) (character 72)) (end (line 44) (character 92)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::exbi::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 44) (character 30)) (end (line 44) (character 52))) (parent (node (document "d0") (qualified-name "SIPrefixes::exbi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 44) (character 30)) (end (line 44) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::exbi::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 44) (character 53)) (end (line 44) (character 71))) (parent (node (document "d0") (qualified-name "SIPrefixes::exbi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 44) (character 53)) (end (line 44) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::femto"))) (kind "attribute def") (name "femto") (declared-name "femto") (range (start (line 16) (character 1)) (end (line 16) (character 104))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::femto::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 16) (character 73)) (end (line 16) (character 102))) (parent (node (document "d0") (qualified-name "SIPrefixes::femto"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 16) (character 73)) (end (line 16) (character 93)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::femto::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 16) (character 31)) (end (line 16) (character 54))) (parent (node (document "d0") (qualified-name "SIPrefixes::femto"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 16) (character 31)) (end (line 16) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::femto::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 16) (character 55)) (end (line 16) (character 72))) (parent (node (document "d0") (qualified-name "SIPrefixes::femto"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 16) (character 55)) (end (line 16) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::gibi"))) (kind "attribute def") (name "gibi") (declared-name "gibi") (range (start (line 41) (character 1)) (end (line 41) (character 104))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::gibi::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 41) (character 72)) (end (line 41) (character 102))) (parent (node (document "d0") (qualified-name "SIPrefixes::gibi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 41) (character 72)) (end (line 41) (character 92)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::gibi::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 41) (character 30)) (end (line 41) (character 52))) (parent (node (document "d0") (qualified-name "SIPrefixes::gibi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 41) (character 30)) (end (line 41) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::gibi::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 41) (character 53)) (end (line 41) (character 71))) (parent (node (document "d0") (qualified-name "SIPrefixes::gibi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 41) (character 53)) (end (line 41) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::giga"))) (kind "attribute def") (name "giga") (declared-name "giga") (range (start (line 27) (character 1)) (end (line 27) (character 100))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::giga::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 27) (character 71)) (end (line 27) (character 98))) (parent (node (document "d0") (qualified-name "SIPrefixes::giga"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 27) (character 71)) (end (line 27) (character 91)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::giga::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 27) (character 30)) (end (line 27) (character 52))) (parent (node (document "d0") (qualified-name "SIPrefixes::giga"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 27) (character 30)) (end (line 27) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::giga::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 27) (character 53)) (end (line 27) (character 70))) (parent (node (document "d0") (qualified-name "SIPrefixes::giga"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 27) (character 53)) (end (line 27) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::hecto"))) (kind "attribute def") (name "hecto") (declared-name "hecto") (range (start (line 24) (character 1)) (end (line 24) (character 102))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::hecto::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 24) (character 73)) (end (line 24) (character 100))) (parent (node (document "d0") (qualified-name "SIPrefixes::hecto"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 24) (character 73)) (end (line 24) (character 93)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::hecto::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 24) (character 31)) (end (line 24) (character 54))) (parent (node (document "d0") (qualified-name "SIPrefixes::hecto"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 24) (character 31)) (end (line 24) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::hecto::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 24) (character 55)) (end (line 24) (character 72))) (parent (node (document "d0") (qualified-name "SIPrefixes::hecto"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 24) (character 55)) (end (line 24) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::kibi"))) (kind "attribute def") (name "kibi") (declared-name "kibi") (range (start (line 39) (character 1)) (end (line 39) (character 102))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::kibi::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 39) (character 72)) (end (line 39) (character 100))) (parent (node (document "d0") (qualified-name "SIPrefixes::kibi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 39) (character 72)) (end (line 39) (character 92)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::kibi::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 39) (character 30)) (end (line 39) (character 52))) (parent (node (document "d0") (qualified-name "SIPrefixes::kibi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 39) (character 30)) (end (line 39) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::kibi::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 39) (character 53)) (end (line 39) (character 71))) (parent (node (document "d0") (qualified-name "SIPrefixes::kibi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 39) (character 53)) (end (line 39) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::kilo"))) (kind "attribute def") (name "kilo") (declared-name "kilo") (range (start (line 25) (character 1)) (end (line 25) (character 100))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::kilo::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 25) (character 71)) (end (line 25) (character 98))) (parent (node (document "d0") (qualified-name "SIPrefixes::kilo"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 25) (character 71)) (end (line 25) (character 91)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::kilo::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 25) (character 30)) (end (line 25) (character 52))) (parent (node (document "d0") (qualified-name "SIPrefixes::kilo"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 25) (character 30)) (end (line 25) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::kilo::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 25) (character 53)) (end (line 25) (character 70))) (parent (node (document "d0") (qualified-name "SIPrefixes::kilo"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 25) (character 53)) (end (line 25) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::mebi"))) (kind "attribute def") (name "mebi") (declared-name "mebi") (range (start (line 40) (character 1)) (end (line 40) (character 104))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::mebi::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 40) (character 72)) (end (line 40) (character 102))) (parent (node (document "d0") (qualified-name "SIPrefixes::mebi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 40) (character 72)) (end (line 40) (character 92)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::mebi::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 40) (character 30)) (end (line 40) (character 52))) (parent (node (document "d0") (qualified-name "SIPrefixes::mebi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 40) (character 30)) (end (line 40) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::mebi::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 40) (character 53)) (end (line 40) (character 71))) (parent (node (document "d0") (qualified-name "SIPrefixes::mebi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 40) (character 53)) (end (line 40) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::mega"))) (kind "attribute def") (name "mega") (declared-name "mega") (range (start (line 26) (character 1)) (end (line 26) (character 100))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::mega::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 26) (character 71)) (end (line 26) (character 98))) (parent (node (document "d0") (qualified-name "SIPrefixes::mega"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 26) (character 71)) (end (line 26) (character 91)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::mega::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 26) (character 30)) (end (line 26) (character 52))) (parent (node (document "d0") (qualified-name "SIPrefixes::mega"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 26) (character 30)) (end (line 26) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::mega::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 26) (character 53)) (end (line 26) (character 70))) (parent (node (document "d0") (qualified-name "SIPrefixes::mega"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 26) (character 53)) (end (line 26) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::micro"))) (kind "attribute def") (name "micro") (declared-name "micro") (range (start (line 19) (character 1)) (end (line 19) (character 104))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::micro::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 19) (character 74)) (end (line 19) (character 102))) (parent (node (document "d0") (qualified-name "SIPrefixes::micro"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 19) (character 74)) (end (line 19) (character 94)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::micro::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 19) (character 31)) (end (line 19) (character 54))) (parent (node (document "d0") (qualified-name "SIPrefixes::micro"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 19) (character 31)) (end (line 19) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::micro::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 19) (character 55)) (end (line 19) (character 73))) (parent (node (document "d0") (qualified-name "SIPrefixes::micro"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 19) (character 55)) (end (line 19) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::milli"))) (kind "attribute def") (name "milli") (declared-name "milli") (range (start (line 20) (character 1)) (end (line 20) (character 103))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::milli::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 20) (character 73)) (end (line 20) (character 101))) (parent (node (document "d0") (qualified-name "SIPrefixes::milli"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 20) (character 73)) (end (line 20) (character 93)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::milli::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 20) (character 31)) (end (line 20) (character 54))) (parent (node (document "d0") (qualified-name "SIPrefixes::milli"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 20) (character 31)) (end (line 20) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::milli::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 20) (character 55)) (end (line 20) (character 72))) (parent (node (document "d0") (qualified-name "SIPrefixes::milli"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 20) (character 55)) (end (line 20) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::nano"))) (kind "attribute def") (name "nano") (declared-name "nano") (range (start (line 18) (character 1)) (end (line 18) (character 101))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::nano::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 18) (character 71)) (end (line 18) (character 99))) (parent (node (document "d0") (qualified-name "SIPrefixes::nano"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 18) (character 71)) (end (line 18) (character 91)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::nano::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 18) (character 30)) (end (line 18) (character 52))) (parent (node (document "d0") (qualified-name "SIPrefixes::nano"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 18) (character 30)) (end (line 18) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::nano::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 18) (character 53)) (end (line 18) (character 70))) (parent (node (document "d0") (qualified-name "SIPrefixes::nano"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 18) (character 53)) (end (line 18) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::pebi"))) (kind "attribute def") (name "pebi") (declared-name "pebi") (range (start (line 43) (character 1)) (end (line 43) (character 104))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::pebi::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 43) (character 72)) (end (line 43) (character 102))) (parent (node (document "d0") (qualified-name "SIPrefixes::pebi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 43) (character 72)) (end (line 43) (character 92)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::pebi::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 43) (character 30)) (end (line 43) (character 52))) (parent (node (document "d0") (qualified-name "SIPrefixes::pebi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 43) (character 30)) (end (line 43) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::pebi::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 43) (character 53)) (end (line 43) (character 71))) (parent (node (document "d0") (qualified-name "SIPrefixes::pebi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 43) (character 53)) (end (line 43) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::peta"))) (kind "attribute def") (name "peta") (declared-name "peta") (range (start (line 29) (character 1)) (end (line 29) (character 101))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::peta::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 29) (character 71)) (end (line 29) (character 99))) (parent (node (document "d0") (qualified-name "SIPrefixes::peta"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 29) (character 71)) (end (line 29) (character 91)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::peta::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 29) (character 30)) (end (line 29) (character 52))) (parent (node (document "d0") (qualified-name "SIPrefixes::peta"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 29) (character 30)) (end (line 29) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::peta::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 29) (character 53)) (end (line 29) (character 70))) (parent (node (document "d0") (qualified-name "SIPrefixes::peta"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 29) (character 53)) (end (line 29) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::pico"))) (kind "attribute def") (name "pico") (declared-name "pico") (range (start (line 17) (character 1)) (end (line 17) (character 102))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::pico::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 17) (character 71)) (end (line 17) (character 100))) (parent (node (document "d0") (qualified-name "SIPrefixes::pico"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 17) (character 71)) (end (line 17) (character 91)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::pico::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 17) (character 30)) (end (line 17) (character 52))) (parent (node (document "d0") (qualified-name "SIPrefixes::pico"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 17) (character 30)) (end (line 17) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::pico::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 17) (character 53)) (end (line 17) (character 70))) (parent (node (document "d0") (qualified-name "SIPrefixes::pico"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 17) (character 53)) (end (line 17) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::tebi"))) (kind "attribute def") (name "tebi") (declared-name "tebi") (range (start (line 42) (character 1)) (end (line 42) (character 104))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::tebi::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 42) (character 72)) (end (line 42) (character 102))) (parent (node (document "d0") (qualified-name "SIPrefixes::tebi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 42) (character 72)) (end (line 42) (character 92)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::tebi::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 42) (character 30)) (end (line 42) (character 52))) (parent (node (document "d0") (qualified-name "SIPrefixes::tebi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 42) (character 30)) (end (line 42) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::tebi::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 42) (character 53)) (end (line 42) (character 71))) (parent (node (document "d0") (qualified-name "SIPrefixes::tebi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 42) (character 53)) (end (line 42) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::tera"))) (kind "attribute def") (name "tera") (declared-name "tera") (range (start (line 28) (character 1)) (end (line 28) (character 101))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::tera::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 28) (character 71)) (end (line 28) (character 99))) (parent (node (document "d0") (qualified-name "SIPrefixes::tera"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 28) (character 71)) (end (line 28) (character 91)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::tera::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 28) (character 30)) (end (line 28) (character 52))) (parent (node (document "d0") (qualified-name "SIPrefixes::tera"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 28) (character 30)) (end (line 28) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::tera::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 28) (character 53)) (end (line 28) (character 70))) (parent (node (document "d0") (qualified-name "SIPrefixes::tera"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 28) (character 53)) (end (line 28) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::yobi"))) (kind "attribute def") (name "yobi") (declared-name "yobi") (range (start (line 46) (character 1)) (end (line 46) (character 104))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::yobi::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 46) (character 72)) (end (line 46) (character 102))) (parent (node (document "d0") (qualified-name "SIPrefixes::yobi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 46) (character 72)) (end (line 46) (character 92)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::yobi::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 46) (character 30)) (end (line 46) (character 52))) (parent (node (document "d0") (qualified-name "SIPrefixes::yobi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 46) (character 30)) (end (line 46) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::yobi::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 46) (character 53)) (end (line 46) (character 71))) (parent (node (document "d0") (qualified-name "SIPrefixes::yobi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 46) (character 53)) (end (line 46) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::yocto"))) (kind "attribute def") (name "yocto") (declared-name "yocto") (range (start (line 13) (character 1)) (end (line 13) (character 104))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::yocto::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 13) (character 73)) (end (line 13) (character 102))) (parent (node (document "d0") (qualified-name "SIPrefixes::yocto"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 13) (character 73)) (end (line 13) (character 93)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::yocto::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 13) (character 31)) (end (line 13) (character 54))) (parent (node (document "d0") (qualified-name "SIPrefixes::yocto"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 13) (character 31)) (end (line 13) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::yocto::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 13) (character 55)) (end (line 13) (character 72))) (parent (node (document "d0") (qualified-name "SIPrefixes::yocto"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 13) (character 55)) (end (line 13) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::yotta"))) (kind "attribute def") (name "yotta") (declared-name "yotta") (range (start (line 32) (character 1)) (end (line 32) (character 103))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::yotta::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 32) (character 73)) (end (line 32) (character 101))) (parent (node (document "d0") (qualified-name "SIPrefixes::yotta"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 32) (character 73)) (end (line 32) (character 93)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::yotta::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 32) (character 31)) (end (line 32) (character 54))) (parent (node (document "d0") (qualified-name "SIPrefixes::yotta"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 32) (character 31)) (end (line 32) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::yotta::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 32) (character 55)) (end (line 32) (character 72))) (parent (node (document "d0") (qualified-name "SIPrefixes::yotta"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 32) (character 55)) (end (line 32) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::zebi"))) (kind "attribute def") (name "zebi") (declared-name "zebi") (range (start (line 45) (character 1)) (end (line 45) (character 104))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::zebi::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 45) (character 72)) (end (line 45) (character 102))) (parent (node (document "d0") (qualified-name "SIPrefixes::zebi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 45) (character 72)) (end (line 45) (character 92)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::zebi::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 45) (character 30)) (end (line 45) (character 52))) (parent (node (document "d0") (qualified-name "SIPrefixes::zebi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 45) (character 30)) (end (line 45) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::zebi::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 45) (character 53)) (end (line 45) (character 71))) (parent (node (document "d0") (qualified-name "SIPrefixes::zebi"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 45) (character 53)) (end (line 45) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::zepto"))) (kind "attribute def") (name "zepto") (declared-name "zepto") (range (start (line 14) (character 1)) (end (line 14) (character 104))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::zepto::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 14) (character 73)) (end (line 14) (character 102))) (parent (node (document "d0") (qualified-name "SIPrefixes::zepto"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 14) (character 73)) (end (line 14) (character 93)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::zepto::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 14) (character 31)) (end (line 14) (character 54))) (parent (node (document "d0") (qualified-name "SIPrefixes::zepto"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 14) (character 31)) (end (line 14) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::zepto::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 14) (character 55)) (end (line 14) (character 72))) (parent (node (document "d0") (qualified-name "SIPrefixes::zepto"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 14) (character 55)) (end (line 14) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::zetta"))) (kind "attribute def") (name "zetta") (declared-name "zetta") (range (start (line 31) (character 1)) (end (line 31) (character 103))) (parent (node (document "d0") (qualified-name "SIPrefixes"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitPrefix") (range none)))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::zetta::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (range (start (line 31) (character 73)) (end (line 31) (character 101))) (parent (node (document "d0") (qualified-name "SIPrefixes::zetta"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "conversionFactor") (range (start (line 31) (character 73)) (end (line 31) (character 93)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::zetta::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (range (start (line 31) (character 31)) (end (line 31) (character 54))) (parent (node (document "d0") (qualified-name "SIPrefixes::zetta"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "longName") (range (start (line 31) (character 31)) (end (line 31) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "SIPrefixes::zetta::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (range (start (line 31) (character 55)) (end (line 31) (character 72))) (parent (node (document "d0") (qualified-name "SIPrefixes::zetta"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "symbol") (range (start (line 31) (character 55)) (end (line 31) (character 65)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (range (start (line 6) (character 16)) (end (line 6) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::atto"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::atto::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 15) (character 71)) (end (line 15) (character 91))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::atto::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::atto::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 15) (character 30)) (end (line 15) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::atto::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::atto::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 15) (character 53)) (end (line 15) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::atto::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::centi"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::centi::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 21) (character 73)) (end (line 21) (character 93))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::centi::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::centi::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 21) (character 31)) (end (line 21) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::centi::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::centi::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 21) (character 55)) (end (line 21) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::centi::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::deca"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::deca::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 23) (character 72)) (end (line 23) (character 92))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::deca::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::deca::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 23) (character 30)) (end (line 23) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::deca::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::deca::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 23) (character 53)) (end (line 23) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::deca::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::deci"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::deci::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 22) (character 71)) (end (line 22) (character 91))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::deci::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::deci::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 22) (character 30)) (end (line 22) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::deci::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::deci::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 22) (character 53)) (end (line 22) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::deci::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::exa"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::exa::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 30) (character 69)) (end (line 30) (character 89))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::exa::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::exa::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 30) (character 29)) (end (line 30) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::exa::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::exa::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 30) (character 51)) (end (line 30) (character 61))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::exa::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::exbi"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::exbi::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 44) (character 72)) (end (line 44) (character 92))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::exbi::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::exbi::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 44) (character 30)) (end (line 44) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::exbi::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::exbi::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 44) (character 53)) (end (line 44) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::exbi::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::femto"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::femto::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 16) (character 73)) (end (line 16) (character 93))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::femto::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::femto::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 16) (character 31)) (end (line 16) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::femto::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::femto::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 16) (character 55)) (end (line 16) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::femto::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::gibi"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::gibi::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 41) (character 72)) (end (line 41) (character 92))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::gibi::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::gibi::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 41) (character 30)) (end (line 41) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::gibi::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::gibi::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 41) (character 53)) (end (line 41) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::gibi::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::giga"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::giga::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 27) (character 71)) (end (line 27) (character 91))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::giga::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::giga::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 27) (character 30)) (end (line 27) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::giga::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::giga::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 27) (character 53)) (end (line 27) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::giga::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::hecto"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::hecto::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 24) (character 73)) (end (line 24) (character 93))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::hecto::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::hecto::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 24) (character 31)) (end (line 24) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::hecto::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::hecto::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 24) (character 55)) (end (line 24) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::hecto::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::kibi"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::kibi::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 39) (character 72)) (end (line 39) (character 92))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::kibi::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::kibi::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 39) (character 30)) (end (line 39) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::kibi::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::kibi::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 39) (character 53)) (end (line 39) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::kibi::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::kilo"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::kilo::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 25) (character 71)) (end (line 25) (character 91))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::kilo::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::kilo::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 25) (character 30)) (end (line 25) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::kilo::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::kilo::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 25) (character 53)) (end (line 25) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::kilo::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::mebi"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::mebi::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 40) (character 72)) (end (line 40) (character 92))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::mebi::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::mebi::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 40) (character 30)) (end (line 40) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::mebi::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::mebi::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 40) (character 53)) (end (line 40) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::mebi::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::mega"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::mega::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 26) (character 71)) (end (line 26) (character 91))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::mega::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::mega::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 26) (character 30)) (end (line 26) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::mega::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::mega::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 26) (character 53)) (end (line 26) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::mega::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::micro"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::micro::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 19) (character 74)) (end (line 19) (character 94))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::micro::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::micro::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 19) (character 31)) (end (line 19) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::micro::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::micro::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 19) (character 55)) (end (line 19) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::micro::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::milli"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::milli::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 20) (character 73)) (end (line 20) (character 93))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::milli::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::milli::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 20) (character 31)) (end (line 20) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::milli::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::milli::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 20) (character 55)) (end (line 20) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::milli::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::nano"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::nano::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 18) (character 71)) (end (line 18) (character 91))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::nano::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::nano::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 18) (character 30)) (end (line 18) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::nano::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::nano::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 18) (character 53)) (end (line 18) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::nano::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::pebi"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::pebi::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 43) (character 72)) (end (line 43) (character 92))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::pebi::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::pebi::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 43) (character 30)) (end (line 43) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::pebi::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::pebi::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 43) (character 53)) (end (line 43) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::pebi::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::peta"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::peta::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 29) (character 71)) (end (line 29) (character 91))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::peta::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::peta::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 29) (character 30)) (end (line 29) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::peta::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::peta::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 29) (character 53)) (end (line 29) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::peta::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::pico"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::pico::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 17) (character 71)) (end (line 17) (character 91))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::pico::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::pico::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 17) (character 30)) (end (line 17) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::pico::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::pico::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 17) (character 53)) (end (line 17) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::pico::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::tebi"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::tebi::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 42) (character 72)) (end (line 42) (character 92))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::tebi::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::tebi::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 42) (character 30)) (end (line 42) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::tebi::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::tebi::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 42) (character 53)) (end (line 42) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::tebi::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::tera"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::tera::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 28) (character 71)) (end (line 28) (character 91))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::tera::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::tera::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 28) (character 30)) (end (line 28) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::tera::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::tera::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 28) (character 53)) (end (line 28) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::tera::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::yobi"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::yobi::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 46) (character 72)) (end (line 46) (character 92))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::yobi::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::yobi::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 46) (character 30)) (end (line 46) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::yobi::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::yobi::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 46) (character 53)) (end (line 46) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::yobi::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::yocto"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::yocto::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 13) (character 73)) (end (line 13) (character 93))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::yocto::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::yocto::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 13) (character 31)) (end (line 13) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::yocto::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::yocto::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 13) (character 55)) (end (line 13) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::yocto::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::yotta"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::yotta::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 32) (character 73)) (end (line 32) (character 93))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::yotta::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::yotta::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 32) (character 31)) (end (line 32) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::yotta::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::yotta::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 32) (character 55)) (end (line 32) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::yotta::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::zebi"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::zebi::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 45) (character 72)) (end (line 45) (character 92))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::zebi::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::zebi::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 45) (character 30)) (end (line 45) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::zebi::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::zebi::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 45) (character 53)) (end (line 45) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::zebi::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::zepto"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::zepto::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 14) (character 73)) (end (line 14) (character 93))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::zepto::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::zepto::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 14) (character 31)) (end (line 14) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::zepto::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::zepto::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 14) (character 55)) (end (line 14) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::zepto::symbol")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::zetta"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::zetta::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "conversionFactor") (range (start (line 31) (character 73)) (end (line 31) (character 93))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::zetta::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::zetta::longName"))) (kind redefinition) (ordinal 0)) (authored-target "longName") (range (start (line 31) (character 31)) (end (line 31) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::zetta::longName")))))
    (reference (id (source (node (document "d0") (qualified-name "SIPrefixes::zetta::symbol"))) (kind redefinition) (ordinal 0)) (authored-target "symbol") (range (start (line 31) (character 55)) (end (line 31) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SIPrefixes::zetta::symbol")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::atto::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::atto::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::atto::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::atto::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::atto::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::atto::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::atto::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::atto::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::atto::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::centi::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::centi::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::centi::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::centi::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::centi::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::centi::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::centi::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::centi::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::centi::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::deca::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::deca::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::deca::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::deca::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::deca::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::deca::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::deca::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::deca::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::deca::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::deci::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::deci::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::deci::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::deci::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::deci::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::deci::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::deci::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::deci::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::deci::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::exa::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::exa::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::exa::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::exa::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::exa::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::exa::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::exa::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::exa::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::exa::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::exbi::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::exbi::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::exbi::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::exbi::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::exbi::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::exbi::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::exbi::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::exbi::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::exbi::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::femto::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::femto::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::femto::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::femto::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::femto::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::femto::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::femto::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::femto::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::femto::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::gibi::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::gibi::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::gibi::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::gibi::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::gibi::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::gibi::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::gibi::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::gibi::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::gibi::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::giga::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::giga::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::giga::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::giga::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::giga::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::giga::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::giga::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::giga::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::giga::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::hecto::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::hecto::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::hecto::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::hecto::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::hecto::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::hecto::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::hecto::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::hecto::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::hecto::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::kibi::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::kibi::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::kibi::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::kibi::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::kibi::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::kibi::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::kibi::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::kibi::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::kibi::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::kilo::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::kilo::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::kilo::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::kilo::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::kilo::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::kilo::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::kilo::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::kilo::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::kilo::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::mebi::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::mebi::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::mebi::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::mebi::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::mebi::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::mebi::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::mebi::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::mebi::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::mebi::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::mega::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::mega::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::mega::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::mega::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::mega::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::mega::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::mega::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::mega::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::mega::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::micro::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::micro::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::micro::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::micro::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::micro::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::micro::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::micro::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::micro::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::micro::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::milli::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::milli::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::milli::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::milli::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::milli::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::milli::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::milli::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::milli::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::milli::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::nano::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::nano::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::nano::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::nano::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::nano::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::nano::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::nano::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::nano::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::nano::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::pebi::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::pebi::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::pebi::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::pebi::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::pebi::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::pebi::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::pebi::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::pebi::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::pebi::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::peta::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::peta::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::peta::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::peta::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::peta::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::peta::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::peta::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::peta::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::peta::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::pico::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::pico::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::pico::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::pico::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::pico::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::pico::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::pico::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::pico::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::pico::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::tebi::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::tebi::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::tebi::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::tebi::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::tebi::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::tebi::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::tebi::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::tebi::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::tebi::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::tera::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::tera::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::tera::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::tera::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::tera::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::tera::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::tera::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::tera::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::tera::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::yobi::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::yobi::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::yobi::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::yobi::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::yobi::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::yobi::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::yobi::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::yobi::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::yobi::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::yocto::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::yocto::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::yocto::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::yocto::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::yocto::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::yocto::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::yocto::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::yocto::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::yocto::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::yotta::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::yotta::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::yotta::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::yotta::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::yotta::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::yotta::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::yotta::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::yotta::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::yotta::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::zebi::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::zebi::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::zebi::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::zebi::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::zebi::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::zebi::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::zebi::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::zebi::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::zebi::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::zepto::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::zepto::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::zepto::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::zepto::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::zepto::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::zepto::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::zepto::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::zepto::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::zepto::symbol"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::zetta::conversionFactor"))) (target (node (document "d0") (qualified-name "SIPrefixes::zetta::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::zetta::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::zetta::longName"))) (target (node (document "d0") (qualified-name "SIPrefixes::zetta::longName"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::zetta::longName"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SIPrefixes::zetta::symbol"))) (target (node (document "d0") (qualified-name "SIPrefixes::zetta::symbol"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SIPrefixes::zetta::symbol"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
