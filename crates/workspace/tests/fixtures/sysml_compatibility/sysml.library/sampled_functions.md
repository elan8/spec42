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
    doc /*
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
        doc /*
		 * SamplePair is a key-value pair of a domain-value and a range-value, used as a sample element in SampledFunction.
		 */

        attribute domainValue :>> key;
        attribute rangeValue :>> val;
    }

    attribute def SampledFunction :> OrderedMap {
        doc /*
	     * SampledFunction is a variable-size, ordered collection of 'SamplePair' elements that represents a generic, discretely sampled, 
	     * uni-variate or multi-variate mathematical function. The function must be montonic, either strictly increasing or strictly
	     * decreasing.
	     * 
	     * It maps discrete domain values to discrete range values.
	     * The domain of the function is represented by the sequence of 'domainValue' of each 'SamplePair' in 'samples', and 
	     * the range of the function is represented by the sequence of 'rangeValue' of each 'SamplePair' in 'samples'.
	     */

        attribute samples : SamplePair :>> elements [0..*] ordered;

        assert constraint {
            // Note: Assumes the functions '<' and '>' are defined for the domain type.
            = (1 .. size(samples) - 1)->forAll { in i; (samples.domainValue#(i) < samples.domainValue#(i+1)) } or (1 .. size(samples) - 1)->forAll { in i; (samples.domainValue#(i) > samples.domainValue#(i+1)) };
        }
    }

    calc def Domain {
        doc /* 
		 * Domain returns the sequence of the domainValues of all samples in a SampledFunction.
		 */

        in fn : SampledFunction;
        return : Anything[0..*] = fn.samples.domainValue;
    }

    calc def Range {
        doc /* 
		 * Range returns the sequence of the rangeValues of all samples in a SampledFunction.
		 */

        in fn : SampledFunction;
        return : Anything[0..*] = fn.samples.rangeValue;
    }

    calc def Sample {
        doc /* 
		 * Sample returns a SampledFunction that samples a given calculation over a sequence of domainValues.
		 */

        in calc calculation {
            in x;
        }
        in attribute domainValues [0..*];
        return sampling = new SampledFunction (
			samples = domainValues->collect { in x; new SamplePair(x, calculation(x)) }
		);
    }

    calc def Interpolate {
        doc /*
		 * An Interpolate calculation returns an interpolated range value from a given SampledFunction for a given domain value.
		 * If the input domain value is outside the bounds of the domainValues of the SampleFunction, null is returned.
		 */

        in attribute fn : SampledFunction;
        in attribute value;
        return attribute result;
    }

    calc interpolateLinear : Interpolate {
        doc /*
		 * interpolateLinear is an Interpolate calculation assuming a linear functional form between SamplePairs.
		 */

        in attribute fn : SampledFunction;
        in attribute value;

        private attribute domainValues = Domain(fn);
        private attribute index : Positive [0..1] = (1..size(domainValues))->select { in i : Positive; domainValues#(i) <= value }#(1);

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
(model
  (namespace
    (library_package 'SampledFunctions'
      (documentation)
      (membership_import private -> 'Base::Anything'[unresolved])
      (membership_import private -> 'ScalarValues::Positive'[unresolved])
      (membership_import private -> 'Collections::KeyValuePair'[unresolved])
      (membership_import private -> 'Collections::OrderedMap'[unresolved])
      (membership_import private -> 'SequenceFunctions::size'[unresolved])
      (membership_import private -> 'ControlFunctions::forAll'[unresolved])
      (membership_import private -> 'ControlFunctions::collect'[unresolved])
      (membership_import private -> 'ControlFunctions::select'[unresolved])
      (attribute_def 'SamplePair' :> 'KeyValuePair'[unresolved]
        (documentation)
        (attribute_usage composite 'domainValue' :>> 'key'[unresolved])
        (attribute_usage composite 'rangeValue' :>> 'val'[unresolved]))
      (attribute_def 'SampledFunction' :> 'OrderedMap'[unresolved]
        (documentation)
        (attribute_usage composite ordered 'samples' : 'SampledFunctions::SamplePair'[attribute_def] :>> 'elements'[unresolved]
          (multiplicity_range [0..*]))
        (assert_constraint_usage
          (result_expr_membership)))
      (calculation_def 'Domain'
        (documentation)
        (reference_usage in reference 'fn' : 'SampledFunctions::SampledFunction'[attribute_def])
        (return_parameter_membership
          (feature_def out : 'Anything'[unresolved]
            (multiplicity_range [0..*])
            (feature_value (=)))))
      (calculation_def 'Range'
        (documentation)
        (reference_usage in reference 'fn' : 'SampledFunctions::SampledFunction'[attribute_def])
        (return_parameter_membership
          (feature_def out : 'Anything'[unresolved]
            (multiplicity_range [0..*])
            (feature_value (=)))))
      (calculation_def 'Sample'
        (documentation)
        (calculation_usage in 'calculation'
          (reference_usage in reference 'x'))
        (attribute_usage in 'domainValues'
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out 'sampling'
            (feature_value (=)))))
      (calculation_def 'Interpolate'
        (documentation)
        (attribute_usage in 'fn' : 'SampledFunctions::SampledFunction'[attribute_def])
        (attribute_usage in 'value')
        (return_parameter_membership
          (attribute_usage out 'result')))
      (calculation_usage 'interpolateLinear' : 'SampledFunctions::Interpolate'[calculation_def]
        (documentation)
        (attribute_usage in 'fn' : 'SampledFunctions::SampledFunction'[attribute_def])
        (attribute_usage in 'value')
        (attribute_usage composite 'domainValues'
          (feature_value (=)))
        (attribute_usage composite 'index' : 'Positive'[unresolved]
          (multiplicity_range [0..1])
          (feature_value (=)))
        (calculation_def 'Linear'
          (attribute_usage in 'lowerSample' : 'SampledFunctions::SamplePair'[attribute_def])
          (attribute_usage in 'upperSample' : 'SampledFunctions::SamplePair'[attribute_def])
          (attribute_usage in 'value')
          (attribute_usage composite 'f'
            (feature_value (=)))
          (return_parameter_membership
            (feature_def out 'result'
              (feature_value (=)))))
        (return_parameter_membership
          (feature_def out 'result'
            (multiplicity_range [0..1])
            (feature_value (=))))))))
~~~
