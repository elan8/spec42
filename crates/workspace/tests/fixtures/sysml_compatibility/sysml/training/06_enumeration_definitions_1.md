# META
~~~ini
description=SysML Training 06 (Enumeration Definitions): Enumeration Definitions-1
type=file
~~~
# SOURCE
~~~sysml
package 'Enumeration Definitions-1' {
	private import ScalarValues::Real;
	
	enum def TrafficLightColor {
		enum green;
		enum yellow;
		enum red;
	}
	
	part def TrafficLight {
		attribute currentColor : TrafficLightColor;
	}
	
	part def TrafficLightGo specializes TrafficLight {
		attribute redefines currentColor = TrafficLightColor::green;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwEnum,KwDef,Ident,OpenCurly,
KwEnum,Ident,Semicolon,
KwEnum,Ident,Semicolon,
KwEnum,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwAttribute,KwRedefines,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Enumeration Definitions-1''
    (import_decl private 'ScalarValues::Real')
    (enum_def 'TrafficLightColor'
      (enum_value 'green')
      (enum_value 'yellow')
      (enum_value 'red'))
    (part_def 'TrafficLight'
      (attribute_usage 'currentColor' : 'TrafficLightColor'))
    (part_def 'TrafficLightGo' :> 'TrafficLight'
      (attribute_usage :>> 'currentColor' value))))
~~~
# FORMAT
~~~sysml
package 'Enumeration Definitions-1' {
    private import ScalarValues::Real;

    enum def TrafficLightColor {
        enum green;
        enum yellow;
        enum red;
    }

    part def TrafficLight {
        attribute currentColor : TrafficLightColor;
    }

    part def TrafficLightGo specializes TrafficLight {
        attribute redefines currentColor = TrafficLightColor::green;
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
# SMG
~~~
(model
  (namespace
    (package 'Enumeration Definitions-1'
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (enum_def 'TrafficLightColor'
        (enum_usage composite 'green')
        (enum_usage composite 'yellow')
        (enum_usage composite 'red'))
      (part_def 'TrafficLight'
        (attribute_usage composite 'currentColor' : 'Enumeration Definitions-1::TrafficLightColor'[enum_def]))
      (part_def 'TrafficLightGo' :> 'Enumeration Definitions-1::TrafficLight'[part_def]
        (attribute_usage composite :>> 'Enumeration Definitions-1::TrafficLight::currentColor'[attribute_usage]
          (feature_value (=)))))))
~~~
