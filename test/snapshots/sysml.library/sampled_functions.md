# META
~~~ini
description=Standard Library: Domain Libraries/Analysis/SampledFunctions
type=file
~~~
# SOURCE
~~~sysml
standard library package SampledFunctions {
	doc
	/*
	 * This package provides a library model of discretely sampled mathematical functions.
	 */

	private import Base::Anything;
	private import ScalarValues::Positive;
	private import Collections::KeyValuePair;
	private import Collections::OrderedMap;
	private import SequenceFunctions::size;
	private import ControlFunctions::forAll;
	private import ControlFunctions::collect;
	private import ControlFunctions::select;
	
    attribute def SamplePair :> KeyValuePair {
		doc
		/*
		 * SamplePair is a key-value pair of a domain-value and a range-value, used as a sample element in SampledFunction.
		 */
	
        attribute domainValue :>> key;
        attribute rangeValue :>> val;
    }

	attribute def SampledFunction :> OrderedMap {
		doc
		/*
	     * SampledFunction is a variable-size, ordered collection of 'SamplePair' elements that represents a generic, discretely sampled, 
	     * uni-variate or multi-variate mathematical function. The function must be montonic, either strictly increasing or strictly
	     * decreasing.
	     * 
	     * It maps discrete domain values to discrete range values.
	     * The domain of the function is represented by the sequence of 'domainValue' of each 'SamplePair' in 'samples', and 
	     * the range of the function is represented by the sequence of 'rangeValue' of each 'SamplePair' in 'samples'.
	     */
	
		attribute samples: SamplePair[0..*] ordered :>> elements;
		
		assert constraint {
			// Note: Assumes the functions '<' and '>' are defined for the domain type.
			(1..size(samples)-1)->forAll { in i; (samples.domainValue#(i) < samples.domainValue#(i+1)) } or  // Strictly increasing
            (1..size(samples)-1)->forAll { in i; (samples.domainValue#(i) > samples.domainValue#(i+1)) }     // Strictly decreasing
		}
	}
	
	calc def Domain { 
		doc
		/* 
		 * Domain returns the sequence of the domainValues of all samples in a SampledFunction.
		 */
		 
		in fn : SampledFunction; 
		return : Anything[0..*] = fn.samples.domainValue;
	}
	
	calc def Range { 
		doc
		/* 
		 * Range returns the sequence of the rangeValues of all samples in a SampledFunction.
		 */
			
		in fn : SampledFunction; 
		return : Anything[0..*] = fn.samples.rangeValue;
	}
	
	calc def Sample {
		doc
		/* 
		 * Sample returns a SampledFunction that samples a given calculation over a sequence of domainValues.
		 */
		 
		in calc calculation { in x; }
		in attribute domainValues [0..*];
		return sampling = new SampledFunction (
			samples = domainValues->collect { in x; new SamplePair(x, calculation(x)) }
		);
	}
	
	calc def Interpolate {
		doc
		/*
		 * An Interpolate calculation returns an interpolated range value from a given SampledFunction for a given domain value.
		 * If the input domain value is outside the bounds of the domainValues of the SampleFunction, null is returned.
		 */
	
		in attribute fn : SampledFunction;
		in attribute value;
		return attribute result;
	}
		
	calc interpolateLinear : Interpolate {
		doc
		/*
		 * interpolateLinear is an Interpolate calculation assuming a linear functional form between SamplePairs.
		 */
	
		in attribute fn : SampledFunction;
		in attribute value;
		
		private attribute domainValues = Domain(fn);
		private attribute index : Positive[0..1] =
			(1..size(domainValues))->select { in i : Positive; domainValues#(i) <= value }#(1);
			
		private calc def Linear {
			in attribute lowerSample : SamplePair;
			in attribute upperSample : SamplePair;
			in attribute value;
			private attribute f = (value - lowerSample.domainValue) / (lowerSample.domainValue - upperSample.domainValue);
			return result = upperSample.rangeValue + f * (lowerSample.rangeValue - upperSample.rangeValue);				
		}
		
		return result [0..1] =
			if index == null or index == size(domainValues)? null
			else if domainValues#(index) < domainValues#(index+1)? Linear(fn.samples#(index), fn.samples#(index+1), value)
			else Linear(fn.samples#(index+1), fn.samples#(index), value);
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sampled_functions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 15 32) (end 15 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 21 34) (end 21 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 22 33) (end 22 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 25 34) (end 25 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 37 50) (end 37 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 39 2) (end 43 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 46 1) (end 54 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 56 1) (end 64 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 66 1) (end 77 2))
      )
      (diagnostic
        (severity error)
        (code "recovered_calc_body_element")
        (source "parser")
        (range (start 72 2) (end 73 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 79 1) (end 89 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 91 1) (end 116 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:1636fc5a3a196fef891bfa1348935c76e2f9eab7265ee64586fed8e7fddf64fb") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Base::Anything") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Positive") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Collections::KeyValuePair") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Collections::OrderedMap") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::size") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::forAll") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::collect") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::select") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "KeyValuePair"))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair::domainValue"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "key"))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair::rangeValue"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "val"))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "OrderedMap"))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction::samples"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SamplePair")) (redefinition (reference "elements"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Positive")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Collections::KeyValuePair")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Collections::OrderedMap")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::collect")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::select")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair"))) (kind specialization) (ordinal 0))
      (authored-target "KeyValuePair")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair::domainValue"))) (kind redefinition) (ordinal 0))
      (authored-target "key")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair::rangeValue"))) (kind redefinition) (ordinal 0))
      (authored-target "val")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction"))) (kind specialization) (ordinal 0))
      (authored-target "OrderedMap")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction::samples"))) (kind featureTyping) (ordinal 0))
      (authored-target "SamplePair")
      (outcome (status resolved) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair")))))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction::samples"))) (kind redefinition) (ordinal 0))
      (authored-target "elements")
      (outcome (status unsupported)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction::samples"))) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction::samples"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sampled_functions.md") (range (start 6 16) (end 6 30)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 7 16) (end 7 38)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Positive")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 8 16) (end 8 41)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Collections::KeyValuePair")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 9 16) (end 9 39)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Collections::OrderedMap")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 10 16) (end 10 39)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 11 16) (end 11 40)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 12 16) (end 12 41)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::collect")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 13 16) (end 13 40)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::select")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 15 32) (end 15 44)) (probe (position 15 32))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair"))) (kind specialization) (ordinal 0) (authored-target "KeyValuePair")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 21 34) (end 21 37)) (probe (position 21 34))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair::domainValue"))) (kind redefinition) (ordinal 0) (authored-target "key")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 22 33) (end 22 36)) (probe (position 22 33))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair::rangeValue"))) (kind redefinition) (ordinal 0) (authored-target "val")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 25 34) (end 25 44)) (probe (position 25 34))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction"))) (kind specialization) (ordinal 0) (authored-target "OrderedMap")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 37 21) (end 37 31)) (probe (position 37 21))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction::samples"))) (kind featureTyping) (ordinal 0) (authored-target "SamplePair")
      (outcome (status resolved) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair")))))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 37 50) (end 37 58)) (probe (position 37 50))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction::samples"))) (kind redefinition) (ordinal 0) (authored-target "elements")
      (outcome (status unsupported)))
  )
)
~~~
