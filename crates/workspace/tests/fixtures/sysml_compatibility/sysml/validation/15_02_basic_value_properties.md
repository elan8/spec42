# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_02-Basic Value Properties
type=file
~~~
# SOURCE
~~~sysml
package '15_02-Basic Value Properties' {
	private import ScalarValues::*;
	
    attribute def LengthValue :> Real {
		doc
		/*
		 * Real world user models would use a quantity type
		 * from the library model. A attribute def is defined
		 * here to show that it is possible.
		 */
	}

    part def Tire {
    	attribute manufacturer: String;
        attribute hubDiameter: LengthValue;
        attribute width: Integer;
    }
    
    part frenchTire: Tire {
    	attribute :>> manufacturer = "Michelin";
    	attribute :>> hubDiameter = 18.0;
    	attribute :>> width = 245;
    }
    
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,StringValue,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''15_02-Basic Value Properties''
    (import_decl private 'ScalarValues::*')
    (attribute_def 'LengthValue' :> 'Real'
      (documentation))
    (part_def 'Tire'
      (attribute_usage 'manufacturer' : 'String')
      (attribute_usage 'hubDiameter' : 'LengthValue')
      (attribute_usage 'width' : 'Integer'))
    (part_usage 'frenchTire' : 'Tire'
      (attribute_usage :>> 'manufacturer' value)
      (attribute_usage :>> 'hubDiameter' value)
      (attribute_usage :>> 'width' value))))
~~~
# FORMAT
~~~sysml
package '15_02-Basic Value Properties' {
    private import ScalarValues::*;

    attribute def LengthValue :> Real {
        doc /*
		 * Real world user models would use a quantity type
		 * from the library model. A attribute def is defined
		 * here to show that it is possible.
		 */
    }

    part def Tire {
        attribute manufacturer : String;
        attribute hubDiameter : LengthValue;
        attribute width : Integer;
    }

    part frenchTire : Tire {
        attribute :>> manufacturer = "Michelin";
        attribute :>> hubDiameter = 18.0;
        attribute :>> width = 245;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Integer'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Integer'
~~~
# SMG
~~~
(model
  (namespace
    (package '15_02-Basic Value Properties'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (attribute_def 'LengthValue' :> 'Real'[unresolved]
        (documentation))
      (part_def 'Tire'
        (attribute_usage composite 'manufacturer' : 'String'[unresolved])
        (attribute_usage composite 'hubDiameter' : '15_02-Basic Value Properties::LengthValue'[attribute_def])
        (attribute_usage composite 'width' : 'Integer'[unresolved]))
      (part_usage 'frenchTire' : '15_02-Basic Value Properties::Tire'[part_def]
        (attribute_usage composite :>> '15_02-Basic Value Properties::Tire::manufacturer'[attribute_usage]
          (feature_value (=)))
        (attribute_usage composite :>> '15_02-Basic Value Properties::Tire::hubDiameter'[attribute_usage]
          (feature_value (=)))
        (attribute_usage composite :>> '15_02-Basic Value Properties::Tire::width'[attribute_usage]
          (feature_value (=)))))))
~~~
