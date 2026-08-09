# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Data Type Library/ScalarValues
type=file
~~~
# SOURCE
~~~kerml
standard library package ScalarValues {
    doc /*
	 * This package contains a basic set of primitive scalar (non-collection) data types. 
	 * These include Boolean and String types and a hierarchy of concrete Number types, from 
	 * the most general type of Complex numbers to the most specific type of Positive integers.</p>
	 */

    private import Base::DataValue;

    abstract datatype ScalarValue specializes DataValue;
    datatype Boolean specializes ScalarValue;
    datatype String specializes ScalarValue;
    abstract datatype NumericalValue specializes ScalarValue;

    abstract datatype Number specializes NumericalValue;
    datatype Complex specializes Number;
    datatype Real specializes Complex;
    datatype Rational specializes Real;
    datatype Integer specializes Rational;
    datatype Natural specializes Integer;
    datatype Positive specializes Natural;
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'DataValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'DataValue'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
KwAbstract,KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
KwAbstract,KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
KwDatatype,Ident,KwSpecializes,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ScalarValues'
    (documentation)
    (import_decl private 'Base::DataValue')
    (datatype_def abstract 'ScalarValue' :> 'DataValue')
    (datatype_def 'Boolean' :> 'ScalarValue')
    (datatype_def 'String' :> 'ScalarValue')
    (datatype_def abstract 'NumericalValue' :> 'ScalarValue')
    (datatype_def abstract 'Number' :> 'NumericalValue')
    (datatype_def 'Complex' :> 'Number')
    (datatype_def 'Real' :> 'Complex')
    (datatype_def 'Rational' :> 'Real')
    (datatype_def 'Integer' :> 'Rational')
    (datatype_def 'Natural' :> 'Integer')
    (datatype_def 'Positive' :> 'Natural')))
~~~
# FORMAT
~~~sysml
standard library package ScalarValues {
    doc /*
	 * This package contains a basic set of primitive scalar (non-collection) data types. 
	 * These include Boolean and String types and a hierarchy of concrete Number types, from 
	 * the most general type of Complex numbers to the most specific type of Positive integers.</p>
	 */

    private import Base::DataValue;

    abstract datatype ScalarValue specializes DataValue;
    datatype Boolean specializes ScalarValue;
    datatype String specializes ScalarValue;
    abstract datatype NumericalValue specializes ScalarValue;

    abstract datatype Number specializes NumericalValue;
    datatype Complex specializes Number;
    datatype Real specializes Complex;
    datatype Rational specializes Real;
    datatype Integer specializes Rational;
    datatype Natural specializes Integer;
    datatype Positive specializes Natural;
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'ScalarValues'
      (documentation)
      (membership_import private -> 'Base::DataValue'[unresolved])
      (datatype_def abstract 'ScalarValue' :> 'DataValue'[unresolved])
      (datatype_def 'Boolean' :> 'ScalarValues::ScalarValue'[datatype_def])
      (datatype_def 'String' :> 'ScalarValues::ScalarValue'[datatype_def])
      (datatype_def abstract 'NumericalValue' :> 'ScalarValues::ScalarValue'[datatype_def])
      (datatype_def abstract 'Number' :> 'ScalarValues::NumericalValue'[datatype_def])
      (datatype_def 'Complex' :> 'ScalarValues::Number'[datatype_def])
      (datatype_def 'Real' :> 'ScalarValues::Complex'[datatype_def])
      (datatype_def 'Rational' :> 'ScalarValues::Real'[datatype_def])
      (datatype_def 'Integer' :> 'ScalarValues::Rational'[datatype_def])
      (datatype_def 'Natural' :> 'ScalarValues::Integer'[datatype_def])
      (datatype_def 'Positive' :> 'ScalarValues::Natural'[datatype_def]))))
~~~
