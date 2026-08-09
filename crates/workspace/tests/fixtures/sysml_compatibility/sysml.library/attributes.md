# META
~~~ini
description=Standard Library: Systems Library/Attributes
type=file
~~~
# SOURCE
~~~sysml
standard library package Attributes {
    doc /*
 * This package defines the base types for attributes and related structural elements 
 * in the SysML language.
 */

    private import Base::DataValue;
    private import Base::dataValues;

    alias AttributeValue for DataValue {
        doc /*
		 * AttributeValue is the most general type of data values that represent qualities or characteristics 
		 * of a system or part of a system. AttributeValue is the base type of all AttributeDefinitions.
		 */
    }

    alias attributeValues for dataValues {
        doc /*
		 * attributeValues is the base feature for all AttributeUsages.
		 */
    }
}
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwAlias,Ident,KwFor,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Attributes'
    (documentation)
    (import_decl private 'Base::DataValue')
    (import_decl private 'Base::dataValues')
    (alias_member 'AttributeValue' for 'DataValue'
      (documentation))
    (alias_member 'attributeValues' for 'dataValues'
      (documentation))))
~~~
# FORMAT
~~~sysml
standard library package Attributes {
    doc /*
 * This package defines the base types for attributes and related structural elements 
 * in the SysML language.
 */

    private import Base::DataValue;
    private import Base::dataValues;

    alias AttributeValue for DataValue {
        doc /*
		 * AttributeValue is the most general type of data values that represent qualities or characteristics 
		 * of a system or part of a system. AttributeValue is the base type of all AttributeDefinitions.
		 */
    }

    alias attributeValues for dataValues {
        doc /*
		 * attributeValues is the base feature for all AttributeUsages.
		 */
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'Attributes'
      (documentation)
      (membership_import private -> 'Base::DataValue'[unresolved])
      (membership_import private -> 'Base::dataValues'[unresolved])
      (alias_member 'AttributeValue' -> 'DataValue'[unresolved])
      (alias_member 'attributeValues' -> 'dataValues'[unresolved]))))
~~~
