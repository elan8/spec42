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
# EXPECTED
~~~
semantic.unresolved_name 'KeyValuePair'
semantic.unresolved_name 'key'
semantic.unresolved_name 'val'
semantic.unresolved_name 'OrderedMap'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Positive'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'KeyValuePair'
semantic.unresolved_name 'key'
semantic.unresolved_name 'val'
semantic.unresolved_name 'OrderedMap'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Positive'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,ColonGtGt,Ident,Semicolon,
KwAttribute,Ident,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,ColonGtGt,Ident,Semicolon,
KwAssert,KwConstraint,OpenCurly,
LineComment,
OpenParen,DecimalValue,DotDot,Ident,OpenParen,Ident,CloseParen,Minus,DecimalValue,CloseParen,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,OpenParen,Ident,Dot,Ident,Hash,OpenParen,Ident,CloseParen,OpenAngle,Ident,Dot,Ident,Hash,OpenParen,Ident,Plus,DecimalValue,CloseParen,CloseParen,CloseCurly,KwOr,LineComment,
OpenParen,DecimalValue,DotDot,Ident,OpenParen,Ident,CloseParen,Minus,DecimalValue,CloseParen,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,OpenParen,Ident,Dot,Ident,Hash,OpenParen,Ident,CloseParen,CloseAngle,Ident,Dot,Ident,Hash,OpenParen,Ident,Plus,DecimalValue,CloseParen,CloseParen,CloseCurly,LineComment,
CloseCurly,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Eq,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Eq,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwCalc,Ident,OpenCurly,KwIn,Ident,Semicolon,CloseCurly,
KwIn,KwAttribute,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwReturn,Ident,Eq,Ident,Ident,OpenParen,
Ident,Eq,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,Ident,Ident,OpenParen,Ident,Comma,Ident,OpenParen,Ident,CloseParen,CloseParen,CloseCurly,
CloseParen,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwIn,KwAttribute,Ident,Semicolon,
KwReturn,KwAttribute,Ident,Semicolon,
CloseCurly,
KwCalc,Ident,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwIn,KwAttribute,Ident,Semicolon,
KwPrivate,KwAttribute,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Eq,
OpenParen,DecimalValue,DotDot,Ident,OpenParen,Ident,CloseParen,CloseParen,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,Ident,Hash,OpenParen,Ident,CloseParen,LtEq,Ident,CloseCurly,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
KwPrivate,KwCalc,KwDef,Ident,OpenCurly,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwIn,KwAttribute,Ident,Semicolon,
KwPrivate,KwAttribute,Ident,Eq,OpenParen,Ident,Minus,Ident,Dot,Ident,CloseParen,Slash,OpenParen,Ident,Dot,Ident,Minus,Ident,Dot,Ident,CloseParen,Semicolon,
KwReturn,Ident,Eq,Ident,Dot,Ident,Plus,Ident,Star,OpenParen,Ident,Dot,Ident,Minus,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwReturn,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Eq,
KwIf,Ident,EqEq,KwNull,KwOr,Ident,EqEq,Ident,OpenParen,Ident,CloseParen,Question,KwNull,
KwElse,KwIf,Ident,Hash,OpenParen,Ident,CloseParen,OpenAngle,Ident,Hash,OpenParen,Ident,Plus,DecimalValue,CloseParen,Question,Ident,OpenParen,Ident,Dot,Ident,Hash,OpenParen,Ident,CloseParen,Comma,Ident,Dot,Ident,Hash,OpenParen,Ident,Plus,DecimalValue,CloseParen,Comma,Ident,CloseParen,
KwElse,Ident,OpenParen,Ident,Dot,Ident,Hash,OpenParen,Ident,Plus,DecimalValue,CloseParen,Comma,Ident,Dot,Ident,Hash,OpenParen,Ident,CloseParen,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'SampledFunctions'
    (documentation)
    (import_decl private 'Base::Anything')
    (import_decl private 'ScalarValues::Positive')
    (import_decl private 'Collections::KeyValuePair')
    (import_decl private 'Collections::OrderedMap')
    (import_decl private 'SequenceFunctions::size')
    (import_decl private 'ControlFunctions::forAll')
    (import_decl private 'ControlFunctions::collect')
    (import_decl private 'ControlFunctions::select')
    (attribute_def 'SamplePair' :> 'KeyValuePair'
      (documentation)
      (attribute_usage 'domainValue' :>> 'key')
      (attribute_usage 'rangeValue' :>> 'val'))
    (attribute_def 'SampledFunction' :> 'OrderedMap'
      (documentation)
      (attribute_usage 'samples' : 'SamplePair' :>> 'elements' multiplicity ordered)
      (sysml_decl
        (line_comment)
        (result_expr_member)))
    (calc_def 'Domain'
      (documentation)
      (default_ref_usage in 'fn' : 'SampledFunction')
      (return_member))
    (calc_def 'Range'
      (documentation)
      (default_ref_usage in 'fn' : 'SampledFunction')
      (return_member))
    (calc_def 'Sample'
      (documentation)
      (calc_usage in 'calculation'
        (default_ref_usage in 'x'))
      (attribute_usage in 'domainValues' multiplicity)
      (return_member))
    (calc_def 'Interpolate'
      (documentation)
      (attribute_usage in 'fn' : 'SampledFunction')
      (attribute_usage in 'value')
      (return_member))
    (calc_usage 'interpolateLinear' : 'Interpolate'
      (documentation)
      (attribute_usage in 'fn' : 'SampledFunction')
      (attribute_usage in 'value')
      (attribute_usage private 'domainValues' value)
      (attribute_usage private 'index' : 'Positive' multiplicity value)
      (calc_def private 'Linear'
        (attribute_usage in 'lowerSample' : 'SamplePair')
        (attribute_usage in 'upperSample' : 'SamplePair')
        (attribute_usage in 'value')
        (attribute_usage private 'f' value)
        (return_member))
      (return_member))))
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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "SampledFunctions"))) (name "SampledFunctions") (declared-name "SampledFunctions")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "SampledFunctions::Anything"))) (name "Anything") (declared-name "Anything"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "SampledFunctions::Domain"))) (name "Domain") (declared-name "Domain")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "SampledFunctions::Domain::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SampledFunctions::Domain")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SampledFunctions::Domain::fn"))) (name "fn") (declared-name "fn") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SampledFunctions::Domain")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "SampledFunctions::Interpolate"))) (name "Interpolate") (declared-name "Interpolate") (declared (own-expression (expression (kind "featureReference") (reference "attribute")))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference")))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "SampledFunctions::Interpolate::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SampledFunctions::Interpolate")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SampledFunctions::Interpolate::fn"))) (name "fn") (declared-name "fn") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SampledFunctions::Interpolate")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SampledFunctions::Interpolate::value"))) (name "value") (declared-name "value") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SampledFunctions::Interpolate")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "SampledFunctions::KeyValuePair"))) (name "KeyValuePair") (declared-name "KeyValuePair"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SampledFunctions::OrderedMap"))) (name "OrderedMap") (declared-name "OrderedMap"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SampledFunctions::Positive"))) (name "Positive") (declared-name "Positive"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "SampledFunctions::Range"))) (name "Range") (declared-name "Range")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "SampledFunctions::Range::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SampledFunctions::Range")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SampledFunctions::Range::fn"))) (name "fn") (declared-name "fn") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SampledFunctions::Range")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "SampledFunctions::Sample"))) (name "Sample") (declared-name "Sample") (declared (own-expression (expression (kind "featureReference") (reference "sampling")))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference")))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "SampledFunctions::Sample::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SampledFunctions::Sample")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SampledFunctions::Sample::calc"))) (name "calc") (declared-name "calc") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SampledFunctions::Sample")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SampledFunctions::Sample::domainValues"))) (name "domainValues") (declared-name "domainValues") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SampledFunctions::Sample")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SampledFunctions::SamplePair"))) (name "SamplePair") (declared-name "SamplePair") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "SampledFunctions::SamplePair::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SampledFunctions::SamplePair")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SampledFunctions::SamplePair::domainValue"))) (name "domainValue") (declared-name "domainValue") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SampledFunctions::SamplePair")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SampledFunctions::SamplePair::rangeValue"))) (name "rangeValue") (declared-name "rangeValue") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SampledFunctions::SamplePair")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SampledFunctions::SampledFunction"))) (name "SampledFunction") (declared-name "SampledFunction") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "SampledFunctions::SampledFunction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SampledFunctions::SampledFunction")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SampledFunctions::SampledFunction::samples"))) (name "samples") (declared-name "samples") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SampledFunctions::SampledFunction")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "SampledFunctions::_documentation"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "SampledFunctions::collect"))) (name "collect") (declared-name "collect"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SampledFunctions::forAll"))) (name "forAll") (declared-name "forAll"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear"))) (name "interpolateLinear") (declared-name "interpolateLinear") (declared (own-expression (expression (kind "featureReference") (reference "private")))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference")))
          (contains
            (element (kind "calc def") (id (node (document "d0") (qualified-name "SampledFunctions::Linear"))) (name "Linear") (declared-name "Linear") (declared (own-expression (expression (kind "featureReference") (reference "private")))) (effective (featuring-type (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear")))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference")))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SampledFunctions::Linear::lowerSample"))) (name "lowerSample") (declared-name "lowerSample") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SampledFunctions::Linear")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SampledFunctions::Linear::upperSample"))) (name "upperSample") (declared-name "upperSample") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SampledFunctions::Linear")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SampledFunctions::Linear::value"))) (name "value") (declared-name "value") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SampledFunctions::Linear")))))
              )
            )
            (element (kind "documentation") (id (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear::fn"))) (name "fn") (declared-name "fn") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear::value"))) (name "value") (declared-name "value") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "SampledFunctions::select"))) (name "select") (declared-name "select"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SampledFunctions::size"))) (name "size") (declared-name "size"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SampledFunctions::Domain::_documentation"))) (to (node (document "d0") (qualified-name "SampledFunctions::Domain"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SampledFunctions::Interpolate::_documentation"))) (to (node (document "d0") (qualified-name "SampledFunctions::Interpolate"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SampledFunctions::Range::_documentation"))) (to (node (document "d0") (qualified-name "SampledFunctions::Range"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SampledFunctions::Sample::_documentation"))) (to (node (document "d0") (qualified-name "SampledFunctions::Sample"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SampledFunctions::SamplePair::_documentation"))) (to (node (document "d0") (qualified-name "SampledFunctions::SamplePair"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SampledFunctions::SampledFunction::_documentation"))) (to (node (document "d0") (qualified-name "SampledFunctions::SampledFunction"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SampledFunctions::_documentation"))) (to (node (document "d0") (qualified-name "SampledFunctions"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear::_documentation"))) (to (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SampledFunctions::Domain::fn"))) (to (node (document "d0") (qualified-name "SampledFunctions::SampledFunction"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SampledFunctions::Interpolate::fn"))) (to (node (document "d0") (qualified-name "SampledFunctions::SampledFunction"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SampledFunctions::Linear::lowerSample"))) (to (node (document "d0") (qualified-name "SampledFunctions::SamplePair"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SampledFunctions::Linear::upperSample"))) (to (node (document "d0") (qualified-name "SampledFunctions::SamplePair"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SampledFunctions::Range::fn"))) (to (node (document "d0") (qualified-name "SampledFunctions::SampledFunction"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SampledFunctions::SampledFunction::samples"))) (to (node (document "d0") (qualified-name "SampledFunctions::SamplePair"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear::fn"))) (to (node (document "d0") (qualified-name "SampledFunctions::SampledFunction"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "SampledFunctions::Domain"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "SampledFunctions::Interpolate"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "SampledFunctions::Linear"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "SampledFunctions::Range"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "SampledFunctions::Sample"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "SampledFunctions::SamplePair"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "SampledFunctions::SamplePair::domainValue"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "SampledFunctions::SamplePair::rangeValue"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "SampledFunctions::SampledFunction"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "SampledFunctions::SampledFunction::samples"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "SampledFunctions::interpolateLinear"))) (status missing-prerequisite) (target "Calculations::Calculation"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/sampled_functions.md"
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 4) (end 15 266))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 21 8) (end 21 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 22 8) (end 22 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 1) (end 25 1093))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 37 2) (end 37 59))
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
