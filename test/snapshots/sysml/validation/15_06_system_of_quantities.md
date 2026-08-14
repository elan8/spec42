# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_06-System of Quantities
type=file
~~~
# SOURCE
~~~sysml
package '15_06-System of Quantities' {
    private import ISQ::*;

	/*
	 * A System of Quantities is represented by a model library package.
	 * 
	 * Its structure is modeled after the International System of Quantities (ISQ):
	 * - Quantity dimension is defined as the product of powers of a selected set of base quantities.
	 * - A system of quantities is multi-dimensional space spanned by the powers of its base quantities.
	 * - Any base quantity is modeled as a specialization of a SimpleUnit. Such a specialized SimpleUnit defines one base unit vector 
	 *   (with power one by definition), e.g. MassUnit with symbol M, that establishes a base quantity dimension for the system of quantities, 
	 *	 without committing yet to a particular choice of measurement unit.
	 * - To complete the system of quantities any number of derived quantities can be added.
	 * - A derived quantity is modeled as a specialization of a DerivedUnit. A DerivedUnit is defined in terms of so-called UnitPowerFactors. 
	 *   Each UnitPowerFactor is a combination of a base (or other derived) quantity and an exponent.
	 * - As an example the AccelerationUnit (specialization of DerivedUnit) can be defined as the combination of LengthUnit (symbol L) 
	 *   to the power 1 and TimeUnit (symbol T) to the power -2, so having quantity dimension L¹⋅T⁻².
	 * - A quantity of dimension one is defined as a derived quantity for which the effective exponent for each 
	 *   of its base quantity power factors is zero. Historically a quantity of dimension one was also called a dimensionless quantity.
	 * - A quantity of dimension one may be defined by adding all quantity power factors that cancel out by having positive and negative 
	 *   exponents. Doing so enables distinction between different 'kinds of' quantities of dimension one, e.g:
	 *   angle (L¹⋅L⁻¹), mass ratio (L¹⋅L⁻¹), power ratio (L²⋅M⋅T⁻³⋅L⁻²⋅M⁻¹⋅T³), Mach number (L¹⋅T⁻¹⋅L⁻¹⋅T¹).
	 * 
	 * The International System of Quantities (ISQ) as defined in ISO/IEC 80000 is added as a predefined model library to SysML v2.
	 * However, this does not prevent to model any other system of quantities in another model library and use it.
	 */
	 
	 /*
	  * Above capabilities were implemented in:
      * - standard library Quantities:
      *   TensorQuantityValue, VectorQuantityValue, ScalarQuantityValue,
      *   tensorQuantities, vectorQuantities, scalarQuantities, 
      *   SystemOfQuantities
	  * - standard library MeasurementReferences:
	  *   TensorMeasurementReference, VectorMeasurementReference, ScalarMeasurementReference,
      *   SystemOfUnits
	  * - standard library ISQBase:
	  *   attribute <isq> 'International System of Quantities': SystemOfQuantities in ISQBase
	  */
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/15_06_system_of_quantities.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 25))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:995cec352700c65c8a118098daf0d7815096263336553e186839d574f265786a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/15_06_system_of_quantities.md") (qualified-name "15_06-System of Quantities"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_06_system_of_quantities.md") (path (name "15_06-System of Quantities") (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/15_06_system_of_quantities.md") (path (name "15_06-System of Quantities") (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/15_06_system_of_quantities.md") (range (start 1 19) (end 1 25)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/15_06_system_of_quantities.md") (path (name "15_06-System of Quantities") (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
)
~~~
