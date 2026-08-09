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
(model
  (namespace
    (library_package 'QuantityCalculations'
      (documentation)
      (namespace_import private -> 'ScalarValues'[unresolved])
      (membership_import private -> 'Quantities::ScalarQuantityValue'[unresolved])
      (membership_import private -> 'MeasurementReferences::ScalarMeasurementReference'[unresolved])
      (membership_import private -> 'MeasurementReferences::DimensionOneValue'[unresolved])
      (calculation_def '[' :> 'BaseFunctions::['[unresolved]
        (reference_usage in reference 'num' : 'Number'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'mRef' : 'ScalarMeasurementReference'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out 'quantity' : 'ScalarQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'isZero' :> 'NumericalFunctions::isZero'[unresolved]
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (calculation_def 'isUnit' :> 'NumericalFunctions::isUnit'[unresolved]
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (calculation_def 'abs' :> 'NumericalFunctions::abs'[unresolved]
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def '+' :> 'NumericalFunctions::+'[unresolved]
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'y' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'ScalarQuantityValue'[unresolved])))
      (calculation_def '-' :> 'NumericalFunctions::-'[unresolved]
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved])
        (reference_usage in reference 'y' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'ScalarQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def '*' :> 'NumericalFunctions::*'[unresolved]
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'y' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def '/' :> 'NumericalFunctions::/'[unresolved]
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'y' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def '**' :> 'NumericalFunctions::**'[unresolved]
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'y' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def '^' :> 'NumericalFunctions::^'[unresolved]
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'y' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def '<' :> 'NumericalFunctions::<'[unresolved]
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'y' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def '>' :> 'NumericalFunctions::>'[unresolved]
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'y' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def '<=' :> 'NumericalFunctions::<='[unresolved]
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'y' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def '>=' :> 'NumericalFunctions::>='[unresolved]
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'y' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'max' :> 'NumericalFunctions::max'[unresolved]
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'y' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'min' :> 'NumericalFunctions::min'[unresolved]
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'y' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def '==' :> 'DataFunctions::=='[unresolved]
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'y' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'sqrt'
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'floor'
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'round'
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarQuantityValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'ToString' :> 'BaseFunctions::ToString'[unresolved]
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'String'[unresolved])))
      (calculation_def 'ToInteger'
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'ToRational'
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Rational'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'ToReal'
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'ToDimensionOneValue'
        (reference_usage in reference 'x' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'DimensionOneValue'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'sum' :> 'NumericalFunctions::sum'[unresolved]
        (reference_usage in reference 'collection' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [0..*]))
        (attribute_usage composite 'zero' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (assert_constraint_usage
          (result_expr_membership))
        (return_parameter_membership
          (feature_def out : 'ScalarQuantityValue'[unresolved]
            (feature_value (=)))))
      (calculation_def 'product' :> 'NumericalFunctions::product'[unresolved]
        (reference_usage in reference 'collection' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [0..*]))
        (attribute_usage composite 'one' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (assert_constraint_usage
          (result_expr_membership))
        (return_parameter_membership
          (feature_def out : 'ScalarQuantityValue'[unresolved]
            (feature_value (=)))))
      (calculation_def 'ConvertQuantity'
        (reference_usage in reference 'x' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'targetMRef' : 'ScalarMeasurementReference'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'ScalarQuantityValue'[unresolved]
            (multiplicity_range [1])))))))
~~~
