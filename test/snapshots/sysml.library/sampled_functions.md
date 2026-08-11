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
  (document "sampled_functions.md"
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 37 50) (end 37 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 72 2) (end 72 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 73 2) (end 73 35))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "b78a9fc2eed441add4b15a734520051ab4c74323f5e558dcb4c41189c6415ff3") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "SampledFunctions"))) (kind "package") (name "SampledFunctions") (declared-name "SampledFunctions") (range (start (line 0) (character 0)) (end (line 0) (character 4021))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (range (start (line 6) (character 1)) (end (line 6) (character 31))) (parent (node (document "d0") (qualified-name "SampledFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 30))))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::Domain"))) (kind "calc def") (name "Domain") (declared-name "Domain") (range (start (line 46) (character 1)) (end (line 46) (character 214))) (parent (node (document "d0") (qualified-name "SampledFunctions"))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::Domain::_documentation"))) (kind "documentation") (name "") (range (start (line 46) (character 1)) (end (line 46) (character 214))) (parent (node (document "d0") (qualified-name "SampledFunctions::Domain"))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::Domain::fn"))) (kind "in out parameter") (name "fn") (declared-name "fn") (range (start (line 52) (character 2)) (end (line 52) (character 26))) (parent (node (document "d0") (qualified-name "SampledFunctions::Domain"))) (authored (relationships (typing (reference "SampledFunction") (range none)))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::Interpolate"))) (kind "calc def") (name "Interpolate") (declared-name "Interpolate") (range (start (line 79) (character 1)) (end (line 79) (character 368))) (parent (node (document "d0") (qualified-name "SampledFunctions"))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::Interpolate::_documentation"))) (kind "documentation") (name "") (range (start (line 79) (character 1)) (end (line 79) (character 368))) (parent (node (document "d0") (qualified-name "SampledFunctions::Interpolate"))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::Interpolate::fn"))) (kind "in out parameter") (name "fn") (declared-name "fn") (range (start (line 86) (character 2)) (end (line 86) (character 36))) (parent (node (document "d0") (qualified-name "SampledFunctions::Interpolate"))) (authored (relationships (typing (reference "SampledFunction") (range none)))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::Interpolate::value"))) (kind "in out parameter") (name "value") (declared-name "value") (range (start (line 87) (character 2)) (end (line 87) (character 21))) (parent (node (document "d0") (qualified-name "SampledFunctions::Interpolate"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::KeyValuePair"))) (kind "import") (name "KeyValuePair") (declared-name "KeyValuePair") (range (start (line 8) (character 1)) (end (line 8) (character 42))) (parent (node (document "d0") (qualified-name "SampledFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Collections::KeyValuePair") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 41))))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::Linear"))) (kind "calc def") (name "Linear") (declared-name "Linear") (range (start (line 104) (character 2)) (end (line 104) (character 355))) (parent (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear"))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::Linear::lowerSample"))) (kind "in out parameter") (name "lowerSample") (declared-name "lowerSample") (range (start (line 105) (character 3)) (end (line 105) (character 41))) (parent (node (document "d0") (qualified-name "SampledFunctions::Linear"))) (authored (relationships (typing (reference "SamplePair") (range none)))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::Linear::upperSample"))) (kind "in out parameter") (name "upperSample") (declared-name "upperSample") (range (start (line 106) (character 3)) (end (line 106) (character 41))) (parent (node (document "d0") (qualified-name "SampledFunctions::Linear"))) (authored (relationships (typing (reference "SamplePair") (range none)))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::Linear::value"))) (kind "in out parameter") (name "value") (declared-name "value") (range (start (line 107) (character 3)) (end (line 107) (character 22))) (parent (node (document "d0") (qualified-name "SampledFunctions::Linear"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::OrderedMap"))) (kind "import") (name "OrderedMap") (declared-name "OrderedMap") (range (start (line 9) (character 1)) (end (line 9) (character 40))) (parent (node (document "d0") (qualified-name "SampledFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Collections::OrderedMap") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 39))))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::Positive"))) (kind "import") (name "Positive") (declared-name "Positive") (range (start (line 7) (character 1)) (end (line 7) (character 39))) (parent (node (document "d0") (qualified-name "SampledFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Positive") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 38))))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::Range"))) (kind "calc def") (name "Range") (declared-name "Range") (range (start (line 56) (character 1)) (end (line 56) (character 210))) (parent (node (document "d0") (qualified-name "SampledFunctions"))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::Range::_documentation"))) (kind "documentation") (name "") (range (start (line 56) (character 1)) (end (line 56) (character 210))) (parent (node (document "d0") (qualified-name "SampledFunctions::Range"))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::Range::fn"))) (kind "in out parameter") (name "fn") (declared-name "fn") (range (start (line 62) (character 2)) (end (line 62) (character 26))) (parent (node (document "d0") (qualified-name "SampledFunctions::Range"))) (authored (relationships (typing (reference "SampledFunction") (range none)))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::Sample"))) (kind "calc def") (name "Sample") (declared-name "Sample") (range (start (line 66) (character 1)) (end (line 66) (character 341))) (parent (node (document "d0") (qualified-name "SampledFunctions"))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::Sample::_documentation"))) (kind "documentation") (name "") (range (start (line 66) (character 1)) (end (line 66) (character 341))) (parent (node (document "d0") (qualified-name "SampledFunctions::Sample"))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::Sample::calc"))) (kind "in out parameter") (name "calc") (declared-name "calc") (range (start (line 72) (character 2)) (end (line 72) (character 31))) (parent (node (document "d0") (qualified-name "SampledFunctions::Sample"))) (authored (relationships (typing (reference "calc calculation") (range none)))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::Sample::domainValues"))) (kind "in out parameter") (name "domainValues") (declared-name "domainValues") (range (start (line 73) (character 2)) (end (line 73) (character 35))) (parent (node (document "d0") (qualified-name "SampledFunctions::Sample"))) (authored (relationships (typing (reference "domainValues [0..*]") (range none)))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::SamplePair"))) (kind "attribute def") (name "SamplePair") (declared-name "SamplePair") (range (start (line 15) (character 4)) (end (line 15) (character 266))) (parent (node (document "d0") (qualified-name "SampledFunctions"))) (authored (membership (kind Owning)) (relationships (typing (reference "KeyValuePair") (range none)))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::SamplePair::_documentation"))) (kind "documentation") (name "") (range (start (line 15) (character 4)) (end (line 15) (character 266))) (parent (node (document "d0") (qualified-name "SampledFunctions::SamplePair"))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::SamplePair::domainValue"))) (kind "attribute") (name "domainValue") (declared-name "domainValue") (range (start (line 21) (character 8)) (end (line 21) (character 38))) (parent (node (document "d0") (qualified-name "SampledFunctions::SamplePair"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "key") (range (start (line 21) (character 34)) (end (line 21) (character 37)))))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::SamplePair::rangeValue"))) (kind "attribute") (name "rangeValue") (declared-name "rangeValue") (range (start (line 22) (character 8)) (end (line 22) (character 37))) (parent (node (document "d0") (qualified-name "SampledFunctions::SamplePair"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "val") (range (start (line 22) (character 33)) (end (line 22) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::SampledFunction"))) (kind "attribute def") (name "SampledFunction") (declared-name "SampledFunction") (range (start (line 25) (character 1)) (end (line 25) (character 1093))) (parent (node (document "d0") (qualified-name "SampledFunctions"))) (authored (membership (kind Owning)) (relationships (typing (reference "OrderedMap") (range none)))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::SampledFunction::_documentation"))) (kind "documentation") (name "") (range (start (line 25) (character 1)) (end (line 25) (character 1093))) (parent (node (document "d0") (qualified-name "SampledFunctions::SampledFunction"))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::SampledFunction::samples"))) (kind "attribute") (name "samples") (declared-name "samples") (range (start (line 37) (character 2)) (end (line 37) (character 59))) (parent (node (document "d0") (qualified-name "SampledFunctions::SampledFunction"))) (authored (membership (kind Feature)) (relationships (typing (reference "SamplePair") (range none)) (redefinition (reference "elements") (range (start (line 37) (character 50)) (end (line 37) (character 58)))))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 4021))) (parent (node (document "d0") (qualified-name "SampledFunctions"))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::collect"))) (kind "import") (name "collect") (declared-name "collect") (range (start (line 12) (character 1)) (end (line 12) (character 42))) (parent (node (document "d0") (qualified-name "SampledFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::collect") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 16)) (end (line 12) (character 41))))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (range (start (line 11) (character 1)) (end (line 11) (character 41))) (parent (node (document "d0") (qualified-name "SampledFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 40))))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear"))) (kind "calc def") (name "interpolateLinear") (declared-name "interpolateLinear") (range (start (line 91) (character 1)) (end (line 91) (character 1034))) (parent (node (document "d0") (qualified-name "SampledFunctions"))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear::_documentation"))) (kind "documentation") (name "") (range (start (line 91) (character 1)) (end (line 91) (character 1034))) (parent (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear"))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear::fn"))) (kind "in out parameter") (name "fn") (declared-name "fn") (range (start (line 97) (character 2)) (end (line 97) (character 36))) (parent (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear"))) (authored (relationships (typing (reference "SampledFunction") (range none)))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear::value"))) (kind "in out parameter") (name "value") (declared-name "value") (range (start (line 98) (character 2)) (end (line 98) (character 21))) (parent (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::select"))) (kind "import") (name "select") (declared-name "select") (range (start (line 13) (character 1)) (end (line 13) (character 41))) (parent (node (document "d0") (qualified-name "SampledFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::select") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 13) (character 16)) (end (line 13) (character 40))))))
    (element (id (node (document "d0") (qualified-name "SampledFunctions::size"))) (kind "import") (name "size") (declared-name "size") (range (start (line 10) (character 1)) (end (line 10) (character 40))) (parent (node (document "d0") (qualified-name "SampledFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 39))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (range (start (line 6) (character 16)) (end (line 6) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::Domain::fn"))) (kind featureTyping) (ordinal 0)) (authored-target "SampledFunction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "SampledFunctions::SampledFunction")))))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::Interpolate::fn"))) (kind featureTyping) (ordinal 0)) (authored-target "SampledFunction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "SampledFunctions::SampledFunction")))))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::Interpolate::value"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "SampledFunctions::Interpolate::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::KeyValuePair"))) (kind membershipImport) (ordinal 0)) (authored-target "Collections::KeyValuePair") (range (start (line 8) (character 16)) (end (line 8) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::Linear::lowerSample"))) (kind featureTyping) (ordinal 0)) (authored-target "SamplePair") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "SampledFunctions::SamplePair")))))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::Linear::upperSample"))) (kind featureTyping) (ordinal 0)) (authored-target "SamplePair") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "SampledFunctions::SamplePair")))))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::Linear::value"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::OrderedMap"))) (kind membershipImport) (ordinal 0)) (authored-target "Collections::OrderedMap") (range (start (line 9) (character 16)) (end (line 9) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::Positive"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Positive") (range (start (line 7) (character 16)) (end (line 7) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::Range::fn"))) (kind featureTyping) (ordinal 0)) (authored-target "SampledFunction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "SampledFunctions::SampledFunction")))))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::Sample::calc"))) (kind featureTyping) (ordinal 0)) (authored-target "calc calculation") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::Sample::domainValues"))) (kind featureTyping) (ordinal 0)) (authored-target "domainValues [0..*]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::SamplePair"))) (kind featureTyping) (ordinal 0)) (authored-target "KeyValuePair") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "SampledFunctions::KeyValuePair")))))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::SamplePair::domainValue"))) (kind redefinition) (ordinal 0)) (authored-target "key") (range (start (line 21) (character 34)) (end (line 21) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::SamplePair::rangeValue"))) (kind redefinition) (ordinal 0)) (authored-target "val") (range (start (line 22) (character 33)) (end (line 22) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::SampledFunction"))) (kind featureTyping) (ordinal 0)) (authored-target "OrderedMap") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "SampledFunctions::OrderedMap")))))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::SampledFunction::samples"))) (kind featureTyping) (ordinal 0)) (authored-target "SamplePair") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "SampledFunctions::SamplePair")))))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::SampledFunction::samples"))) (kind redefinition) (ordinal 0)) (authored-target "elements") (range (start (line 37) (character 50)) (end (line 37) (character 58))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::collect"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::collect") (range (start (line 12) (character 16)) (end (line 12) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (range (start (line 11) (character 16)) (end (line 11) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear::fn"))) (kind featureTyping) (ordinal 0)) (authored-target "SampledFunction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "SampledFunctions::SampledFunction")))))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear::value"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::select"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::select") (range (start (line 13) (character 16)) (end (line 13) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SampledFunctions::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (range (start (line 10) (character 16)) (end (line 10) (character 39))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SampledFunctions::Domain::fn"))) (target (node (document "d0") (qualified-name "SampledFunctions::SampledFunction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SampledFunctions::Domain::fn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SampledFunctions::Interpolate::fn"))) (target (node (document "d0") (qualified-name "SampledFunctions::SampledFunction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SampledFunctions::Interpolate::fn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SampledFunctions::Interpolate::value"))) (target (node (document "d0") (qualified-name "SampledFunctions::Interpolate::_documentation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SampledFunctions::Interpolate::value"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SampledFunctions::Linear::lowerSample"))) (target (node (document "d0") (qualified-name "SampledFunctions::SamplePair"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SampledFunctions::Linear::lowerSample"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SampledFunctions::Linear::upperSample"))) (target (node (document "d0") (qualified-name "SampledFunctions::SamplePair"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SampledFunctions::Linear::upperSample"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SampledFunctions::Linear::value"))) (target (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear::_documentation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SampledFunctions::Linear::value"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SampledFunctions::Range::fn"))) (target (node (document "d0") (qualified-name "SampledFunctions::SampledFunction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SampledFunctions::Range::fn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SampledFunctions::SamplePair"))) (target (node (document "d0") (qualified-name "SampledFunctions::KeyValuePair"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SampledFunctions::SamplePair"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SampledFunctions::SampledFunction"))) (target (node (document "d0") (qualified-name "SampledFunctions::OrderedMap"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SampledFunctions::SampledFunction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SampledFunctions::SampledFunction::samples"))) (target (node (document "d0") (qualified-name "SampledFunctions::SamplePair"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SampledFunctions::SampledFunction::samples"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear::fn"))) (target (node (document "d0") (qualified-name "SampledFunctions::SampledFunction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear::fn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear::value"))) (target (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear::_documentation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear::value"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "SampledFunctions::Interpolate")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SampledFunctions::Linear")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SampledFunctions::Sample")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 21 34) (end 21 37)) (probe (position 21 34))
      (reference
        (source (document "d0") (qualified-name "SampledFunctions::SamplePair::domainValue"))
        (kind redefinition) (ordinal 0) (authored-target "key")
        (range (start 21 34) (end 21 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 22 33) (end 22 36)) (probe (position 22 33))
      (reference
        (source (document "d0") (qualified-name "SampledFunctions::SamplePair::rangeValue"))
        (kind redefinition) (ordinal 0) (authored-target "val")
        (range (start 22 33) (end 22 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 37 50) (end 37 58)) (probe (position 37 50))
      (reference
        (source (document "d0") (qualified-name "SampledFunctions::SampledFunction::samples"))
        (kind redefinition) (ordinal 0) (authored-target "elements")
        (range (start 37 50) (end 37 58))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 16) (end 6 30)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "SampledFunctions::Anything"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
        (range (start 6 16) (end 6 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 38)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "SampledFunctions::Positive"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Positive")
        (range (start 7 16) (end 7 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 39)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "SampledFunctions::OrderedMap"))
        (kind membershipImport) (ordinal 0) (authored-target "Collections::OrderedMap")
        (range (start 9 16) (end 9 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 39)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "SampledFunctions::size"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
        (range (start 10 16) (end 10 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 40)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "SampledFunctions::forAll"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
        (range (start 11 16) (end 11 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 16) (end 13 40)) (probe (position 13 16))
      (reference
        (source (document "d0") (qualified-name "SampledFunctions::select"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::select")
        (range (start 13 16) (end 13 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 41)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "SampledFunctions::KeyValuePair"))
        (kind membershipImport) (ordinal 0) (authored-target "Collections::KeyValuePair")
        (range (start 8 16) (end 8 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 16) (end 12 41)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "SampledFunctions::collect"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::collect")
        (range (start 12 16) (end 12 41))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
