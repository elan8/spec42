# META
~~~ini
description=KerML Massed Thing: MassedThings
type=file
~~~
# SOURCE
~~~kerml
private import ScalarValues::*;
package MassedThings {
	
	public class MassedThing {
		public name: String;
		public mass: Real = 0;
	}
	
	public assoc MassedThingAssembly {
		public end [0..1] feature assembly: MassedThing;
		public end [0..*] feature parts: MassedThing;
	}
}
~~~
# TOKENS
~~~zig
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwPublic,KwClass,Ident,OpenCurly,
KwPublic,Ident,Colon,Ident,Semicolon,
KwPublic,Ident,Colon,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwPublic,KwAssoc,Ident,OpenCurly,
KwPublic,KwEnd,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,Semicolon,
KwPublic,KwEnd,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (import_decl private 'ScalarValues::*')
  (package_def 'MassedThings'
    (class_def public 'MassedThing'
      (feature_def public 'name' : 'String')
      (feature_def public 'mass' : 'Real' value))
    (association_def public 'MassedThingAssembly'
      (feature_def public end 'assembly' multiplicity : 'MassedThing')
      (feature_def public end 'parts' multiplicity : 'MassedThing'))))
~~~
# FORMAT
~~~sysml
private import ScalarValues::*;
package MassedThings {
    public class MassedThing {
        public name: String;
        public mass: Real = 0;
    }

    public assoc MassedThingAssembly {
        public end feature assembly[0..1] : MassedThing;
        public end feature parts[0..*] : MassedThing;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'Real'
~~~
# SMG
~~~
(model
  (namespace
    (namespace_import private -> 'ScalarValues'[unresolved])
    (package 'MassedThings'
      (class_def 'MassedThing'
        (feature_def 'name' : 'String'[unresolved])
        (feature_def 'mass' : 'Real'[unresolved]
          (feature_value (=))))
      (association_def 'MassedThingAssembly'
        (feature_def end 'assembly' : 'MassedThings::MassedThing'[class_def]
          (multiplicity_range [0..1]))
        (feature_def end 'parts' : 'MassedThings::MassedThing'[class_def]
          (multiplicity_range [0..*]))))))
~~~
