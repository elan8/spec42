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
  (document "memory://snapshot/15_07_system_of_units_and_scales.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 19) (end 1 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 19) (end 2 38))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:9e006ca56a1ee9db24a8167f8a17a3e6e49185b6b8ff43e498434d446d473dd1") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/15_07_system_of_units_and_scales.md") (qualified-name "15_07-System of Units and Scales"))) (kind package) (membership (kind owning) (visibility default)) (documentation (comment (text "\n\t * A System of Units and Scales is represented by a model library package.\n\t * \n\t * Its structure is modeled after the International System of Units -- Système Internationale d'Unités, abbreviated to SI -- as defined in ISO/IEC 80000:\n\t * - Measurement units and scales are generalized to a common super type MeasurementReference.\n\t * - A particular quantity is modeled as the tuple of a numerical value (i.e. a mathematical number) and a MeasurementReference.\n\t * - An actual measurement unit is modeled as a usage of a specialization of either SimpleUnit or DerivedUnit, e.g. TimeUnit or ForceUnit,\n\t *   see the SI package.\n\t * - The quantity dimension of the actual unit usage must match the quantity dimension of the generic quantity unit definition that it is a usage of.\n\t * - A system of units and scales must define exactly one selected base unit for each base quantity in the associated system of quantities. The collection of \n\t *   base units forms the foundation for automated quantity value conversion between any pair of compatible units and/or scales.\n\t * - If only a measurement unit is used on a quantity value, it implies expression on a ratio scale, in other words only the ratio between the actual quantity value,\n\t *   and the defined unit value is of importance. On ratio scales for one kind of quantity that only differ in their unit (e.g. metre and inch) \n\t *   zero is zero no matter what unit is selected.\n\t * - A unit may carry a conversion factor definition w.r.t. to another reference unit. It can be a conversion by convention (e.g. between metre and foot) or \n\t *   via an ISO/IEC 80000 prefix symbol that indicates a decimal or binary multiple or sub-multiple (e.g. kilo, nano, mega, kibi, mebi, ...). See package SIPrefixes. \n\t * - In addition to measurement units / ratio scales also other types of measurement scales are supported. The additional scales are:\n\t *   - ordinal scales (e.g. Beaufort wind force, Richter Scale, Rockwell C hardness scale), \n\t *   - interval scales (e.g. absolute temperature in deg C or F), \n\t *   - cyclic ratio scales (e.g. rotation angle with modulus 360 degree), \n\t *   - logarithmic scales (e.g. dB(A) or dBA sound pressure level w.r.t. a reference ambient pressure, dB(m) or dBm power ratio w.r.t. 1 mW).\n\t * - Any base unit quantity is modeled as a specialization of a SimpleUnit. This specialized SimpleUnit (e.g. MassUnit) defines one base unit vector (with power one by definition)\n\t *   that establishes a base quantity dimension for the system of quantities, without committing yet to a particular choice of measurement unit.\n\t * \n\t * The International System of Units (SI) as defined in ISO/IEC 80000 as well as the US Customary System of Units as defined by NIST SP 811\n\t * are added as predefined model libraries to SysML v2.\n\t * However, this does not prevent to model any other system of units and scales in another model library and use it.\n\t ")) (comment (text "\n\t  * Above capabilities were implemented in:\n\t  * - standard library MeasurementReferences:\n      *   TensorMeasurementReference, VectorMeasurementReference, ScalarMeasurementReference,\n      *   MeasurementUnit, OrdinalScale, IntervalScale, CyclicRatioScale, LogarithmicScale, \n      *   SystemOfUnits\n\t  * - standard library SI:\n\t  *   attribute <si> 'ISO/IEC 80000 International System of Units' : SystemOfUnits\n      *     :>> systemOfQuantities = isq;\n      *     :>> baseUnits = (m, kg, s, A, K, mol, cd);\n      *   }\n\t  "))))
    (declaration (id (node (document "memory://snapshot/15_07_system_of_units_and_scales.md") (path (named (kind package) (name "15_07-System of Units and Scales")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/15_07_system_of_units_and_scales.md") (path (named (kind package) (name "15_07-System of Units and Scales")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "USCustomaryUnits") (import (shape namespace) (recursive false))))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/15_07_system_of_units_and_scales.md") (path (named (kind package) (name "15_07-System of Units and Scales")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_07_system_of_units_and_scales.md") (path (named (kind package) (name "15_07-System of Units and Scales")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "USCustomaryUnits")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/15_07_system_of_units_and_scales.md") (range (start 1 19) (end 1 25)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/15_07_system_of_units_and_scales.md") (path (named (kind package) (name "15_07-System of Units and Scales")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_07_system_of_units_and_scales.md") (range (start 2 19) (end 2 38)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/15_07_system_of_units_and_scales.md") (path (named (kind package) (name "15_07-System of Units and Scales")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "USCustomaryUnits")
      (outcome (status unresolved)))
    )
  )
)
~~~
