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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 34) (end 21 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
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
        (code "unresolved_reference")
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
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 53 2) (end 53 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 63 2) (end 63 50))
      )
      (diagnostic
        (severity error)
        (code "recovered_calc_body_element")
        (source "parser")
        (range (start 72 2) (end 73 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 74 9) (end 74 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 74 18) (end 76 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 76 2) (end 77 1))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 88 9) (end 88 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 88 19) (end 88 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 100 2) (end 100 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 100 10) (end 100 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 100 20) (end 100 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 100 33) (end 101 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 101 2) (end 101 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 101 10) (end 101 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 101 20) (end 101 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 101 26) (end 102 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 102 81) (end 104 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 108 3) (end 108 10))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 108 11) (end 108 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 108 21) (end 108 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 108 23) (end 109 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 109 10) (end 109 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 109 17) (end 110 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 112 9) (end 112 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 112 16) (end 116 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:1636fc5a3a196fef891bfa1348935c76e2f9eab7265ee64586fed8e7fddf64fb") (contract-version "parser-owned-resolution-v1"))
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
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Domain"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Domain::fn"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SampledFunction") (direction in))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Interpolate"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "attribute")) (expressionOperand (reference "result"))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Interpolate::fn"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SampledFunction") (direction in))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Interpolate::value"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Range"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Range::fn"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SampledFunction") (direction in))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Sample"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "sampling"))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Sample::domainValues"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "KeyValuePair"))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair::domainValue"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "key"))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair::rangeValue"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "val"))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "OrderedMap"))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction::samples"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SamplePair")) (redefinition (reference "elements"))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Interpolate")) (expressionOperand (reference "private")) (expressionOperand (reference "attribute")) (expressionOperand (reference "domainValues")) (expressionOperand (reference "private")) (expressionOperand (reference "attribute")) (expressionOperand (reference "index")) (expressionOperand (reference "result"))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear"))) (kind calc-def) (membership (kind owning) (visibility private)) (authored (membership (kind owning) (visibility private)) (relationships (expressionOperand (reference "private")) (expressionOperand (reference "attribute")) (expressionOperand (reference "f")) (expressionOperand (reference "result"))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear::lowerSample"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SamplePair") (direction in))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear::upperSample"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SamplePair") (direction in))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear::value"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::fn"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SampledFunction") (direction in))))
    (declaration (id (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::value"))) (kind parameter) (membership (kind feature) (visibility default)))
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
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Domain::fn"))) (kind featureTyping) (ordinal 0))
      (authored-target "SampledFunction")
      (outcome (status resolved) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction")))))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Interpolate"))) (kind expressionOperand) (ordinal 0))
      (authored-target "attribute")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Interpolate"))) (kind expressionOperand) (ordinal 1))
      (authored-target "result")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Interpolate::fn"))) (kind featureTyping) (ordinal 0))
      (authored-target "SampledFunction")
      (outcome (status resolved) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction")))))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Range::fn"))) (kind featureTyping) (ordinal 0))
      (authored-target "SampledFunction")
      (outcome (status resolved) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction")))))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Sample"))) (kind expressionOperand) (ordinal 0))
      (authored-target "sampling")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair"))) (kind specialization) (ordinal 0))
      (authored-target "KeyValuePair")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair::domainValue"))) (kind redefinition) (ordinal 0))
      (authored-target "key")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair::rangeValue"))) (kind redefinition) (ordinal 0))
      (authored-target "val")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction"))) (kind specialization) (ordinal 0))
      (authored-target "OrderedMap")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction::samples"))) (kind featureTyping) (ordinal 0))
      (authored-target "SamplePair")
      (outcome (status resolved) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair")))))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction::samples"))) (kind redefinition) (ordinal 0))
      (authored-target "elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (kind featureTyping) (ordinal 0))
      (authored-target "Interpolate")
      (outcome (status resolved) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Interpolate")))))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (kind expressionOperand) (ordinal 0))
      (authored-target "private")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (kind expressionOperand) (ordinal 1))
      (authored-target "attribute")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (kind expressionOperand) (ordinal 2))
      (authored-target "domainValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (kind expressionOperand) (ordinal 3))
      (authored-target "private")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (kind expressionOperand) (ordinal 4))
      (authored-target "attribute")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (kind expressionOperand) (ordinal 5))
      (authored-target "index")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (kind expressionOperand) (ordinal 6))
      (authored-target "result")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear"))) (kind expressionOperand) (ordinal 0))
      (authored-target "private")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear"))) (kind expressionOperand) (ordinal 1))
      (authored-target "attribute")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear"))) (kind expressionOperand) (ordinal 2))
      (authored-target "f")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear"))) (kind expressionOperand) (ordinal 3))
      (authored-target "result")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear::lowerSample"))) (kind featureTyping) (ordinal 0))
      (authored-target "SamplePair")
      (outcome (status resolved) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair")))))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear::upperSample"))) (kind featureTyping) (ordinal 0))
      (authored-target "SamplePair")
      (outcome (status resolved) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair")))))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::fn"))) (kind featureTyping) (ordinal 0))
      (authored-target "SampledFunction")
      (outcome (status resolved) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction")))))
  )
  (relationships
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Domain::fn"))) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Domain::fn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Interpolate::fn"))) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Interpolate::fn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Range::fn"))) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Range::fn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction::samples"))) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction::samples"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Interpolate"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear::lowerSample"))) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear::lowerSample"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear::upperSample"))) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear::upperSample"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::fn"))) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::fn"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Interpolate"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Interpolate"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Sample"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear"))) (value (kind unresolved-operand)))
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
  (query (document "memory://snapshot/sampled_functions.md") (range (start 52 10) (end 52 25)) (probe (position 52 10))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Domain::fn"))) (kind featureTyping) (ordinal 0) (authored-target "SampledFunction")
      (outcome (status resolved) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction")))))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 88 9) (end 88 18)) (probe (position 88 9))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Interpolate"))) (kind expressionOperand) (ordinal 0) (authored-target "attribute")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 88 19) (end 88 25)) (probe (position 88 19))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Interpolate"))) (kind expressionOperand) (ordinal 1) (authored-target "result")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 86 20) (end 86 35)) (probe (position 86 20))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Interpolate::fn"))) (kind featureTyping) (ordinal 0) (authored-target "SampledFunction")
      (outcome (status resolved) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction")))))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 62 10) (end 62 25)) (probe (position 62 10))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Range::fn"))) (kind featureTyping) (ordinal 0) (authored-target "SampledFunction")
      (outcome (status resolved) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction")))))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 74 9) (end 74 17)) (probe (position 74 9))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Sample"))) (kind expressionOperand) (ordinal 0) (authored-target "sampling")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 15 32) (end 15 44)) (probe (position 15 32))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair"))) (kind specialization) (ordinal 0) (authored-target "KeyValuePair")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 21 34) (end 21 37)) (probe (position 21 34))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair::domainValue"))) (kind redefinition) (ordinal 0) (authored-target "key")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 22 33) (end 22 36)) (probe (position 22 33))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair::rangeValue"))) (kind redefinition) (ordinal 0) (authored-target "val")
      (outcome (status unresolved)))
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
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 91 26) (end 91 37)) (probe (position 91 26))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (kind featureTyping) (ordinal 0) (authored-target "Interpolate")
      (outcome (status resolved) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::Interpolate")))))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 100 2) (end 100 9)) (probe (position 100 2))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (kind expressionOperand) (ordinal 0) (authored-target "private")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 100 10) (end 100 19)) (probe (position 100 10))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (kind expressionOperand) (ordinal 1) (authored-target "attribute")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 100 20) (end 100 32)) (probe (position 100 20))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (kind expressionOperand) (ordinal 2) (authored-target "domainValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 101 2) (end 101 9)) (probe (position 101 2))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (kind expressionOperand) (ordinal 3) (authored-target "private")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 101 10) (end 101 19)) (probe (position 101 10))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (kind expressionOperand) (ordinal 4) (authored-target "attribute")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 101 20) (end 101 25)) (probe (position 101 20))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (kind expressionOperand) (ordinal 5) (authored-target "index")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 112 9) (end 112 15)) (probe (position 112 9))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear"))) (kind expressionOperand) (ordinal 6) (authored-target "result")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 108 3) (end 108 10)) (probe (position 108 3))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear"))) (kind expressionOperand) (ordinal 0) (authored-target "private")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 108 11) (end 108 20)) (probe (position 108 11))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear"))) (kind expressionOperand) (ordinal 1) (authored-target "attribute")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 108 21) (end 108 22)) (probe (position 108 21))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear"))) (kind expressionOperand) (ordinal 2) (authored-target "f")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 109 10) (end 109 16)) (probe (position 109 10))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear"))) (kind expressionOperand) (ordinal 3) (authored-target "result")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 105 30) (end 105 40)) (probe (position 105 30))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear::lowerSample"))) (kind featureTyping) (ordinal 0) (authored-target "SamplePair")
      (outcome (status resolved) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair")))))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 106 30) (end 106 40)) (probe (position 106 30))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::Linear::upperSample"))) (kind featureTyping) (ordinal 0) (authored-target "SamplePair")
      (outcome (status resolved) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SamplePair")))))
  )
  (query (document "memory://snapshot/sampled_functions.md") (range (start 97 20) (end 97 35)) (probe (position 97 20))
    (reference (id (source (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::interpolateLinear::fn"))) (kind featureTyping) (ordinal 0) (authored-target "SampledFunction")
      (outcome (status resolved) (target (node (document "memory://snapshot/sampled_functions.md") (qualified-name "SampledFunctions::SampledFunction")))))
  )
)
~~~
