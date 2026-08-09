# META
~~~ini
description=SysML Example (Simple Tests): DefaultValueTest
type=file
~~~
# SOURCE
~~~sysml
package DefaultValueTest {
	
	part def V {
		attribute m default = 10;
		attribute n = 20;
	}
	
	part v1 : V {
		attribute :>> m = 20;
	}
	
	part def W :> V {
		attribute :>> m default = n;
	}
	
	part v2 = new W();
	
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,KwDefault,Eq,DecimalValue,Semicolon,
KwAttribute,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,KwDefault,Eq,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Eq,Ident,Ident,OpenParen,CloseParen,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'DefaultValueTest'
    (part_def 'V'
      (attribute_usage 'm' value)
      (attribute_usage 'n' value))
    (part_usage 'v1' : 'V'
      (attribute_usage :>> 'm' value))
    (part_def 'W' :> 'V'
      (attribute_usage :>> 'm' value))
    (part_usage 'v2' value)))
~~~
# FORMAT
~~~sysml
package DefaultValueTest {
    part def V {
        attribute m default = 10;
        attribute n = 20;
    }

    part v1 : V {
        attribute :>> m = 20;
    }

    part def W :> V {
        attribute :>> m default = n;
    }

    part v2 = new W();
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
# SMG
~~~
(model
  (namespace
    (package 'DefaultValueTest'
      (part_def 'V'
        (attribute_usage composite 'm'
          (feature_value (default =)))
        (attribute_usage composite 'n'
          (feature_value (=))))
      (part_usage 'v1' : 'DefaultValueTest::V'[part_def]
        (attribute_usage composite :>> 'DefaultValueTest::V::m'[attribute_usage]
          (feature_value (=))))
      (part_def 'W' :> 'DefaultValueTest::V'[part_def]
        (attribute_usage composite :>> 'DefaultValueTest::V::m'[attribute_usage]
          (feature_value (default =))))
      (part_usage 'v2'
        (feature_value (=))))))
~~~
