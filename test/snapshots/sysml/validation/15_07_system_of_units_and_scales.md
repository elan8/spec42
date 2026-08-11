# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_07-System of Units and Scales
type=file
~~~
# SOURCE
~~~sysml
package '15_07-System of Units and Scales' {
    private import ISQ::*;
    private import USCustomaryUnits::*;

	/*
	 * A System of Units and Scales is represented by a model library package.
	 * 
	 * Its structure is modeled after the International System of Units -- Système Internationale d'Unités, abbreviated to SI -- as defined in ISO/IEC 80000:
	 * - Measurement units and scales are generalized to a common super type MeasurementReference.
	 * - A particular quantity is modeled as the tuple of a numerical value (i.e. a mathematical number) and a MeasurementReference.
	 * - An actual measurement unit is modeled as a usage of a specialization of either SimpleUnit or DerivedUnit, e.g. TimeUnit or ForceUnit,
	 *   see the SI package.
	 * - The quantity dimension of the actual unit usage must match the quantity dimension of the generic quantity unit definition that it is a usage of.
	 * - A system of units and scales must define exactly one selected base unit for each base quantity in the associated system of quantities. The collection of 
	 *   base units forms the foundation for automated quantity value conversion between any pair of compatible units and/or scales.
	 * - If only a measurement unit is used on a quantity value, it implies expression on a ratio scale, in other words only the ratio between the actual quantity value,
	 *   and the defined unit value is of importance. On ratio scales for one kind of quantity that only differ in their unit (e.g. metre and inch) 
	 *   zero is zero no matter what unit is selected.
	 * - A unit may carry a conversion factor definition w.r.t. to another reference unit. It can be a conversion by convention (e.g. between metre and foot) or 
	 *   via an ISO/IEC 80000 prefix symbol that indicates a decimal or binary multiple or sub-multiple (e.g. kilo, nano, mega, kibi, mebi, ...). See package SIPrefixes. 
	 * - In addition to measurement units / ratio scales also other types of measurement scales are supported. The additional scales are:
	 *   - ordinal scales (e.g. Beaufort wind force, Richter Scale, Rockwell C hardness scale), 
	 *   - interval scales (e.g. absolute temperature in deg C or F), 
	 *   - cyclic ratio scales (e.g. rotation angle with modulus 360 degree), 
	 *   - logarithmic scales (e.g. dB(A) or dBA sound pressure level w.r.t. a reference ambient pressure, dB(m) or dBm power ratio w.r.t. 1 mW).
	 * - Any base unit quantity is modeled as a specialization of a SimpleUnit. This specialized SimpleUnit (e.g. MassUnit) defines one base unit vector (with power one by definition)
	 *   that establishes a base quantity dimension for the system of quantities, without committing yet to a particular choice of measurement unit.
	 * 
	 * The International System of Units (SI) as defined in ISO/IEC 80000 as well as the US Customary System of Units as defined by NIST SP 811
	 * are added as predefined model libraries to SysML v2.
	 * However, this does not prevent to model any other system of units and scales in another model library and use it.
	 */
	 
	 /*
	  * Above capabilities were implemented in:
	  * - standard library MeasurementReferences:
      *   TensorMeasurementReference, VectorMeasurementReference, ScalarMeasurementReference,
      *   MeasurementUnit, OrdinalScale, IntervalScale, CyclicRatioScale, LogarithmicScale, 
      *   SystemOfUnits
	  * - standard library SI:
	  *   attribute <si> 'ISO/IEC 80000 International System of Units' : SystemOfUnits
      *     :>> systemOfQuantities = isq;
      *     :>> baseUnits = (m, kg, s, A, K, mol, cd);
      *   }
	  */
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_07_system_of_units_and_scales.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 19) (end 2 35))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
RegularComment,
RegularComment,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''15_07-System of Units and Scales''
    (import_decl private 'ISQ::*')
    (import_decl private 'USCustomaryUnits::*')
    (comment)
    (comment)))
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# FORMAT
~~~sysml
package '15_07-System of Units and Scales' {
    private import ISQ::*;
    private import USCustomaryUnits::*;

    /*
	 * A System of Units and Scales is represented by a model library package.
	 * 
	 * Its structure is modeled after the International System of Units -- Système Internationale d'Unités, abbreviated to SI -- as defined in ISO/IEC 80000:
	 * - Measurement units and scales are generalized to a common super type MeasurementReference.
	 * - A particular quantity is modeled as the tuple of a numerical value (i.e. a mathematical number) and a MeasurementReference.
	 * - An actual measurement unit is modeled as a usage of a specialization of either SimpleUnit or DerivedUnit, e.g. TimeUnit or ForceUnit,
	 *   see the SI package.
	 * - The quantity dimension of the actual unit usage must match the quantity dimension of the generic quantity unit definition that it is a usage of.
	 * - A system of units and scales must define exactly one selected base unit for each base quantity in the associated system of quantities. The collection of 
	 *   base units forms the foundation for automated quantity value conversion between any pair of compatible units and/or scales.
	 * - If only a measurement unit is used on a quantity value, it implies expression on a ratio scale, in other words only the ratio between the actual quantity value,
	 *   and the defined unit value is of importance. On ratio scales for one kind of quantity that only differ in their unit (e.g. metre and inch) 
	 *   zero is zero no matter what unit is selected.
	 * - A unit may carry a conversion factor definition w.r.t. to another reference unit. It can be a conversion by convention (e.g. between metre and foot) or 
	 *   via an ISO/IEC 80000 prefix symbol that indicates a decimal or binary multiple or sub-multiple (e.g. kilo, nano, mega, kibi, mebi, ...). See package SIPrefixes. 
	 * - In addition to measurement units / ratio scales also other types of measurement scales are supported. The additional scales are:
	 *   - ordinal scales (e.g. Beaufort wind force, Richter Scale, Rockwell C hardness scale), 
	 *   - interval scales (e.g. absolute temperature in deg C or F), 
	 *   - cyclic ratio scales (e.g. rotation angle with modulus 360 degree), 
	 *   - logarithmic scales (e.g. dB(A) or dBA sound pressure level w.r.t. a reference ambient pressure, dB(m) or dBm power ratio w.r.t. 1 mW).
	 * - Any base unit quantity is modeled as a specialization of a SimpleUnit. This specialized SimpleUnit (e.g. MassUnit) defines one base unit vector (with power one by definition)
	 *   that establishes a base quantity dimension for the system of quantities, without committing yet to a particular choice of measurement unit.
	 * 
	 * The International System of Units (SI) as defined in ISO/IEC 80000 as well as the US Customary System of Units as defined by NIST SP 811
	 * are added as predefined model libraries to SysML v2.
	 * However, this does not prevent to model any other system of units and scales in another model library and use it.
	 */

    /*
	  * Above capabilities were implemented in:
	  * - standard library MeasurementReferences:
      *   TensorMeasurementReference, VectorMeasurementReference, ScalarMeasurementReference,
      *   MeasurementUnit, OrdinalScale, IntervalScale, CyclicRatioScale, LogarithmicScale, 
      *   SystemOfUnits
	  * - standard library SI:
	  *   attribute <si> 'ISO/IEC 80000 International System of Units' : SystemOfUnits
      *     :>> systemOfQuantities = isq;
      *     :>> baseUnits = (m, kg, s, A, K, mol, cd);
      *   }
	  */
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "44a6fbcd597ba7dca13c30bd4195ef065484ed824840db86206f793f6e4c3f53") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "15_07-System of Units and Scales"))) (kind "package") (name "15_07-System of Units and Scales") (declared-name "15_07-System of Units and Scales") (range (start (line 0) (character 0)) (end (line 0) (character 3588))))
    (element (id (node (document "d0") (qualified-name "15_07-System of Units and Scales::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 4)) (end (line 1) (character 26))) (parent (node (document "d0") (qualified-name "15_07-System of Units and Scales"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 19)) (end (line 1) (character 22))))))
    (element (id (node (document "d0") (qualified-name "15_07-System of Units and Scales::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 4)) (end (line 2) (character 39))) (parent (node (document "d0") (qualified-name "15_07-System of Units and Scales"))) (authored (membership (kind Import) (visibility "private") (import (reference "USCustomaryUnits::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 19)) (end (line 2) (character 35))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15_07-System of Units and Scales::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 1) (character 19)) (end (line 1) (character 22))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_07-System of Units and Scales::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "USCustomaryUnits::*") (range (start (line 2) (character 19)) (end (line 2) (character 35))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
