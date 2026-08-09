# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/QuantityCalculations
type=file
~~~
# SOURCE
~~~sysml
standard library package QuantityCalculations {
	doc
	/*
	 * This package package defines calculations for the construction of and computations on ScalarQuantityValues.
	 */
	 
	private import ScalarValues::*;
    private import Quantities::ScalarQuantityValue;
    private import MeasurementReferences::ScalarMeasurementReference;
    private import MeasurementReferences::DimensionOneValue;
    
    calc def '[' specializes BaseFunctions::'[' { 
    	in num: Number[1]; 
    	in mRef: ScalarMeasurementReference[1]; 
    	return quantity : ScalarQuantityValue[1];
    }

    calc def isZero specializes NumericalFunctions::isZero { 
    	in x: ScalarQuantityValue[1]; 
        return : Boolean[1] = NumericalFunctions::isZero(x.num);
    }
    calc def isUnit specializes NumericalFunctions::isUnit { 
    	in x: ScalarQuantityValue[1]; 
        return : Boolean[1] = NumericalFunctions::isUnit(x.num);
    }
    
	calc def abs specializes NumericalFunctions::abs { in x: ScalarQuantityValue[1]; return : ScalarQuantityValue[1]; }

	calc def '+' specializes NumericalFunctions::'+' { in x: ScalarQuantityValue[1]; in y: ScalarQuantityValue[0..1]; return : ScalarQuantityValue; }
	calc def '-' specializes NumericalFunctions::'-' { in x: ScalarQuantityValue; in y: ScalarQuantityValue[0..1]; return : ScalarQuantityValue[1]; }
	calc def '*' specializes NumericalFunctions::'*' { in x: ScalarQuantityValue[1]; in y: ScalarQuantityValue[1]; return : ScalarQuantityValue[1]; }
	calc def '/' specializes NumericalFunctions::'/' { in x: ScalarQuantityValue[1]; in y: ScalarQuantityValue[1]; return : ScalarQuantityValue[1]; }
	calc def '**' specializes NumericalFunctions::'**' { in x: ScalarQuantityValue[1]; in y: Real[1]; return : ScalarQuantityValue[1]; }
	calc def '^' specializes NumericalFunctions::'^' { in x: ScalarQuantityValue[1]; in y: Real[1]; return : ScalarQuantityValue[1]; }
	
	calc def '<' specializes NumericalFunctions::'<' { in x: ScalarQuantityValue[1]; in y: ScalarQuantityValue[1]; return : Boolean[1]; }
	calc def '>' specializes NumericalFunctions::'>' { in x: ScalarQuantityValue[1]; in y: ScalarQuantityValue[1]; return : Boolean[1]; }
	calc def '<=' specializes NumericalFunctions::'<=' { in x: ScalarQuantityValue[1]; in y: ScalarQuantityValue[1]; return : Boolean[1]; }
	calc def '>=' specializes NumericalFunctions::'>=' { in x: ScalarQuantityValue[1]; in y: ScalarQuantityValue[1]; return : Boolean[1]; }

	calc def max specializes NumericalFunctions::max { in x: ScalarQuantityValue[1]; in y: ScalarQuantityValue[1]; return : ScalarQuantityValue[1]; }
	calc def min specializes NumericalFunctions::min { in x: ScalarQuantityValue[1]; in y: ScalarQuantityValue[1]; return : ScalarQuantityValue[1]; }

	calc def '==' specializes DataFunctions::'==' { in x: ScalarQuantityValue[1]; in y: ScalarQuantityValue[1]; return : Boolean[1]; }
		
	calc def sqrt{ in x: ScalarQuantityValue[1]; return : ScalarQuantityValue[1]; }

	calc def floor { in x: ScalarQuantityValue[1]; return : ScalarQuantityValue[1]; }
	calc def round { in x: ScalarQuantityValue[1]; return : ScalarQuantityValue[1]; }
	
	calc def ToString specializes BaseFunctions::ToString { in x: ScalarQuantityValue[1]; return : String; }
	calc def ToInteger { in x: ScalarQuantityValue[1]; return : Integer[1]; }
	calc def ToRational { in x: ScalarQuantityValue[1]; return : Rational[1]; }
	calc def ToReal { in x: ScalarQuantityValue[1]; return : Real[1]; }
	calc def ToDimensionOneValue { in x: Real[1]; return : DimensionOneValue[1]; }
	
	calc def sum specializes NumericalFunctions::sum { in collection: ScalarQuantityValue[0..*]; 
        private attribute zero : ScalarQuantityValue[1];
		assert constraint { isZero(zero) }
		return : ScalarQuantityValue = NumericalFunctions::sum0(collection, zero);
	}
	
	calc def product specializes NumericalFunctions::product { in collection: ScalarQuantityValue[0..*]; 
		private attribute one : ScalarQuantityValue[1];
		assert constraint { isUnit(one) }
        return : ScalarQuantityValue = NumericalFunctions::product1(collection, one);
	}

    calc def ConvertQuantity{ in x: ScalarQuantityValue[1]; in targetMRef: ScalarMeasurementReference[1]; return : ScalarQuantityValue[1]; }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'BaseFunctions::['
semantic.unresolved_name 'Number'
semantic.unresolved_name 'ScalarMeasurementReference'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'NumericalFunctions::isZero'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::isUnit'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::abs'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'NumericalFunctions::+'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'NumericalFunctions::-'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'NumericalFunctions::*'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'NumericalFunctions::/'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'NumericalFunctions::**'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'NumericalFunctions::^'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'NumericalFunctions::<'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::>'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::<='
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::>='
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::max'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'NumericalFunctions::min'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'DataFunctions::=='
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'BaseFunctions::ToString'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'String'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'NumericalFunctions::sum'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'NumericalFunctions::product'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarMeasurementReference'
semantic.unresolved_name 'ScalarQuantityValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'BaseFunctions::['
semantic.unresolved_name 'Number'
semantic.unresolved_name 'ScalarMeasurementReference'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'NumericalFunctions::isZero'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::isUnit'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::abs'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'NumericalFunctions::+'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'NumericalFunctions::-'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'NumericalFunctions::*'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'NumericalFunctions::/'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'NumericalFunctions::**'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'NumericalFunctions::^'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'NumericalFunctions::<'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::>'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::<='
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::>='
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'NumericalFunctions::max'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'NumericalFunctions::min'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'DataFunctions::=='
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'BaseFunctions::ToString'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'String'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Rational'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'NumericalFunctions::sum'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'NumericalFunctions::product'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'ScalarMeasurementReference'
semantic.unresolved_name 'ScalarQuantityValue'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwCalc,KwDef,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,ColonColon,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,ColonColon,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,Semicolon,CloseCurly,
KwCalc,KwDef,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAssert,KwConstraint,OpenCurly,Ident,OpenParen,Ident,CloseParen,CloseCurly,
KwReturn,Colon,Ident,Eq,Ident,ColonColon,Ident,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAssert,KwConstraint,OpenCurly,Ident,OpenParen,Ident,CloseParen,CloseCurly,
KwReturn,Colon,Ident,Eq,Ident,ColonColon,Ident,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'QuantityCalculations'
    (documentation)
    (import_decl private 'ScalarValues::*')
    (import_decl private 'Quantities::ScalarQuantityValue')
    (import_decl private 'MeasurementReferences::ScalarMeasurementReference')
    (import_decl private 'MeasurementReferences::DimensionOneValue')
    (calc_def ''['' :> 'BaseFunctions::'[''
      (default_ref_usage in 'num' : 'Number' multiplicity)
      (default_ref_usage in 'mRef' : 'ScalarMeasurementReference' multiplicity)
      (return_member))
    (calc_def 'isZero' :> 'NumericalFunctions::isZero'
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def 'isUnit' :> 'NumericalFunctions::isUnit'
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def 'abs' :> 'NumericalFunctions::abs'
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def ''+'' :> 'NumericalFunctions::'+''
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (default_ref_usage in 'y' : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def ''-'' :> 'NumericalFunctions::'-''
      (default_ref_usage in 'x' : 'ScalarQuantityValue')
      (default_ref_usage in 'y' : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def ''*'' :> 'NumericalFunctions::'*''
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (default_ref_usage in 'y' : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def ''/'' :> 'NumericalFunctions::'/''
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (default_ref_usage in 'y' : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def ''**'' :> 'NumericalFunctions::'**''
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (default_ref_usage in 'y' : 'Real' multiplicity)
      (return_member))
    (calc_def ''^'' :> 'NumericalFunctions::'^''
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (default_ref_usage in 'y' : 'Real' multiplicity)
      (return_member))
    (calc_def ''<'' :> 'NumericalFunctions::'<''
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (default_ref_usage in 'y' : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def ''>'' :> 'NumericalFunctions::'>''
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (default_ref_usage in 'y' : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def ''<='' :> 'NumericalFunctions::'<=''
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (default_ref_usage in 'y' : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def ''>='' :> 'NumericalFunctions::'>=''
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (default_ref_usage in 'y' : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def 'max' :> 'NumericalFunctions::max'
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (default_ref_usage in 'y' : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def 'min' :> 'NumericalFunctions::min'
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (default_ref_usage in 'y' : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def ''=='' :> 'DataFunctions::'==''
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (default_ref_usage in 'y' : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def 'sqrt'
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def 'floor'
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def 'round'
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def 'ToString' :> 'BaseFunctions::ToString'
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def 'ToInteger'
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def 'ToRational'
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def 'ToReal'
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (return_member))
    (calc_def 'ToDimensionOneValue'
      (default_ref_usage in 'x' : 'Real' multiplicity)
      (return_member))
    (calc_def 'sum' :> 'NumericalFunctions::sum'
      (default_ref_usage in 'collection' : 'ScalarQuantityValue' multiplicity)
      (attribute_usage private 'zero' : 'ScalarQuantityValue' multiplicity)
      (sysml_decl
        (result_expr_member))
      (return_member))
    (calc_def 'product' :> 'NumericalFunctions::product'
      (default_ref_usage in 'collection' : 'ScalarQuantityValue' multiplicity)
      (attribute_usage private 'one' : 'ScalarQuantityValue' multiplicity)
      (sysml_decl
        (result_expr_member))
      (return_member))
    (calc_def 'ConvertQuantity'
      (default_ref_usage in 'x' : 'ScalarQuantityValue' multiplicity)
      (default_ref_usage in 'targetMRef' : 'ScalarMeasurementReference' multiplicity)
      (return_member))))
~~~
# FORMAT
~~~sysml
standard library package QuantityCalculations {
    doc /*
	 * This package package defines calculations for the construction of and computations on ScalarQuantityValues.
	 */

    private import ScalarValues::*;
    private import Quantities::ScalarQuantityValue;
    private import MeasurementReferences::ScalarMeasurementReference;
    private import MeasurementReferences::DimensionOneValue;

    calc def '[' specializes BaseFunctions::'[' {
        in num : Number [1];
        in mRef : ScalarMeasurementReference [1];
        return quantity : ScalarQuantityValue[1];
    }

    calc def isZero specializes NumericalFunctions::isZero {
        in x : ScalarQuantityValue [1];
        return : Boolean[1] = NumericalFunctions::isZero(x.num);
    }
    calc def isUnit specializes NumericalFunctions::isUnit {
        in x : ScalarQuantityValue [1];
        return : Boolean[1] = NumericalFunctions::isUnit(x.num);
    }

    calc def abs specializes NumericalFunctions::abs {
        in x : ScalarQuantityValue [1];
        return : ScalarQuantityValue[1];
    }

    calc def '+' specializes NumericalFunctions::'+' {
        in x : ScalarQuantityValue [1];
        in y : ScalarQuantityValue [0..1];
        return : ScalarQuantityValue;
    }
    calc def '-' specializes NumericalFunctions::'-' {
        in x : ScalarQuantityValue;
        in y : ScalarQuantityValue [0..1];
        return : ScalarQuantityValue[1];
    }
    calc def '*' specializes NumericalFunctions::'*' {
        in x : ScalarQuantityValue [1];
        in y : ScalarQuantityValue [1];
        return : ScalarQuantityValue[1];
    }
    calc def '/' specializes NumericalFunctions::'/' {
        in x : ScalarQuantityValue [1];
        in y : ScalarQuantityValue [1];
        return : ScalarQuantityValue[1];
    }
    calc def '**' specializes NumericalFunctions::'**' {
        in x : ScalarQuantityValue [1];
        in y : Real [1];
        return : ScalarQuantityValue[1];
    }
    calc def '^' specializes NumericalFunctions::'^' {
        in x : ScalarQuantityValue [1];
        in y : Real [1];
        return : ScalarQuantityValue[1];
    }

    calc def '<' specializes NumericalFunctions::'<' {
        in x : ScalarQuantityValue [1];
        in y : ScalarQuantityValue [1];
        return : Boolean[1];
    }
    calc def '>' specializes NumericalFunctions::'>' {
        in x : ScalarQuantityValue [1];
        in y : ScalarQuantityValue [1];
        return : Boolean[1];
    }
    calc def '<=' specializes NumericalFunctions::'<=' {
        in x : ScalarQuantityValue [1];
        in y : ScalarQuantityValue [1];
        return : Boolean[1];
    }
    calc def '>=' specializes NumericalFunctions::'>=' {
        in x : ScalarQuantityValue [1];
        in y : ScalarQuantityValue [1];
        return : Boolean[1];
    }

    calc def max specializes NumericalFunctions::max {
        in x : ScalarQuantityValue [1];
        in y : ScalarQuantityValue [1];
        return : ScalarQuantityValue[1];
    }
    calc def min specializes NumericalFunctions::min {
        in x : ScalarQuantityValue [1];
        in y : ScalarQuantityValue [1];
        return : ScalarQuantityValue[1];
    }

    calc def '==' specializes DataFunctions::'==' {
        in x : ScalarQuantityValue [1];
        in y : ScalarQuantityValue [1];
        return : Boolean[1];
    }

    calc def sqrt {
        in x : ScalarQuantityValue [1];
        return : ScalarQuantityValue[1];
    }

    calc def floor {
        in x : ScalarQuantityValue [1];
        return : ScalarQuantityValue[1];
    }
    calc def round {
        in x : ScalarQuantityValue [1];
        return : ScalarQuantityValue[1];
    }

    calc def ToString specializes BaseFunctions::ToString {
        in x : ScalarQuantityValue [1];
        return : String;
    }
    calc def ToInteger {
        in x : ScalarQuantityValue [1];
        return : Integer[1];
    }
    calc def ToRational {
        in x : ScalarQuantityValue [1];
        return : Rational[1];
    }
    calc def ToReal {
        in x : ScalarQuantityValue [1];
        return : Real[1];
    }
    calc def ToDimensionOneValue {
        in x : Real [1];
        return : DimensionOneValue[1];
    }

    calc def sum specializes NumericalFunctions::sum {
        in collection : ScalarQuantityValue [0..*];
        private attribute zero : ScalarQuantityValue [1];
        assert constraint {
            = isZero(zero);
        }
        return : ScalarQuantityValue = NumericalFunctions::sum0(collection, zero);
    }

    calc def product specializes NumericalFunctions::product {
        in collection : ScalarQuantityValue [0..*];
        private attribute one : ScalarQuantityValue [1];
        assert constraint {
            = isUnit(one);
        }
        return : ScalarQuantityValue = NumericalFunctions::product1(collection, one);
    }

    calc def ConvertQuantity {
        in x : ScalarQuantityValue [1];
        in targetMRef : ScalarMeasurementReference [1];
        return : ScalarQuantityValue[1];
    }
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "QuantityCalculations"))) (name "QuantityCalculations") (declared-name "QuantityCalculations")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "QuantityCalculations::*"))) (name "*") (declared-name "*"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::*#calc_def"))) (name "*") (declared-name "*")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::*#calc_def::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::*#calc_def")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::*#calc_def::y"))) (name "y") (declared-name "y") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::*#calc_def")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::**"))) (name "**") (declared-name "**")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::**::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::**")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::**::y"))) (name "y") (declared-name "y") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::**")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::+"))) (name "+") (declared-name "+")
          (contains
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::+::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::+")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::+::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::+")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::+::y"))) (name "y") (declared-name "y") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::+")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::-"))) (name "-") (declared-name "-")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::-::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::-")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::-::y"))) (name "y") (declared-name "y") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::-")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::/"))) (name "/") (declared-name "/")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::/::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::/")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::/::y"))) (name "y") (declared-name "y") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::/")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::<"))) (name "<") (declared-name "<")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::<::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::<")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::<::y"))) (name "y") (declared-name "y") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::<")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::<="))) (name "<=") (declared-name "<=")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::<=::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::<=")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::<=::y"))) (name "y") (declared-name "y") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::<=")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::=="))) (name "==") (declared-name "==")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::==::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::==")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::==::y"))) (name "y") (declared-name "y") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::==")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::>"))) (name ">") (declared-name ">")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::>::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::>")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::>::y"))) (name "y") (declared-name "y") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::>")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::>="))) (name ">=") (declared-name ">=")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::>=::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::>=")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::>=::y"))) (name "y") (declared-name "y") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::>=")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::ConvertQuantity"))) (name "ConvertQuantity") (declared-name "ConvertQuantity")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::ConvertQuantity::targetMRef"))) (name "targetMRef") (declared-name "targetMRef") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::ConvertQuantity")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::ConvertQuantity::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::ConvertQuantity")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "QuantityCalculations::DimensionOneValue"))) (name "DimensionOneValue") (declared-name "DimensionOneValue"))
        (element (kind "import") (id (node (document "d0") (qualified-name "QuantityCalculations::ScalarMeasurementReference"))) (name "ScalarMeasurementReference") (declared-name "ScalarMeasurementReference"))
        (element (kind "import") (id (node (document "d0") (qualified-name "QuantityCalculations::ScalarQuantityValue"))) (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::ToDimensionOneValue"))) (name "ToDimensionOneValue") (declared-name "ToDimensionOneValue")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::ToDimensionOneValue::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::ToDimensionOneValue")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::ToInteger"))) (name "ToInteger") (declared-name "ToInteger")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::ToInteger::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::ToInteger")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::ToRational"))) (name "ToRational") (declared-name "ToRational")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::ToRational::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::ToRational")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::ToReal"))) (name "ToReal") (declared-name "ToReal")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::ToReal::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::ToReal")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::ToString"))) (name "ToString") (declared-name "ToString")
          (contains
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::ToString::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::ToString")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::ToString::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::ToString")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::["))) (name "[") (declared-name "[")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::[::mRef"))) (name "mRef") (declared-name "mRef") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::[")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::[::num"))) (name "num") (declared-name "num") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::[")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::^"))) (name "^") (declared-name "^")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::^::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::^")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::^::y"))) (name "y") (declared-name "y") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::^")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "QuantityCalculations::_documentation"))) (name ""))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::abs"))) (name "abs") (declared-name "abs")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::abs::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::abs")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::floor"))) (name "floor") (declared-name "floor")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::floor::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::floor")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::isUnit"))) (name "isUnit") (declared-name "isUnit")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::isUnit::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::isUnit")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::isZero"))) (name "isZero") (declared-name "isZero")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::isZero::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::isZero")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::max"))) (name "max") (declared-name "max")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::max::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::max")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::max::y"))) (name "y") (declared-name "y") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::max")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::min"))) (name "min") (declared-name "min")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::min::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::min")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::min::y"))) (name "y") (declared-name "y") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::min")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::product"))) (name "product") (declared-name "product")
          (contains
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::product::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::product")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::product::collection"))) (name "collection") (declared-name "collection") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::product")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::round"))) (name "round") (declared-name "round")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::round::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::round")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::sqrt"))) (name "sqrt") (declared-name "sqrt")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::sqrt::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::sqrt")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "QuantityCalculations::sum"))) (name "sum") (declared-name "sum")
          (contains
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::sum::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::sum")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "QuantityCalculations::sum::collection"))) (name "collection") (declared-name "collection") (effective (featuring-type (node (document "d0") (qualified-name "QuantityCalculations::sum")))))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "QuantityCalculations::_documentation"))) (to (node (document "d0") (qualified-name "QuantityCalculations"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
