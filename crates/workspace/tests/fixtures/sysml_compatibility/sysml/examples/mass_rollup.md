# META
~~~ini
description=SysML Example (Mass Roll-up): MassRollup
type=file
~~~
# SOURCE
~~~sysml
package MassRollup {
	private import NumericalFunctions::*;
	
	part def MassedThing {
		attribute mass :> ISQ::mass; 
		attribute totalMass :> ISQ::mass;
	}
	
	part simpleThing : MassedThing {
		attribute redefines totalMass = mass;
	}
	
	part compositeThing : MassedThing {
		part subcomponents: MassedThing[*];
		
		attribute redefines totalMass default
			mass + sum(subcomponents.totalMass); 
	}
	
	part filteredMassThing :> compositeThing {
		abstract attribute minMass :> ISQ::mass;
		
		attribute redefines totalMass =
			mass + sum(subcomponents.totalMass.?{in p :> ISQ::mass; p > minMass});
	}

}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,KwRedefines,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwAttribute,KwRedefines,Ident,KwDefault,
Ident,Plus,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwAbstract,KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwRedefines,Ident,Eq,
Ident,Plus,Ident,OpenParen,Ident,Dot,Ident,DotQuestion,OpenCurly,KwIn,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,Ident,CloseAngle,Ident,CloseCurly,CloseParen,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'MassRollup'
    (import_decl private 'NumericalFunctions::*')
    (part_def 'MassedThing'
      (attribute_usage 'mass' :> 'ISQ::mass')
      (attribute_usage 'totalMass' :> 'ISQ::mass'))
    (part_usage 'simpleThing' : 'MassedThing'
      (attribute_usage :>> 'totalMass' value))
    (part_usage 'compositeThing' : 'MassedThing'
      (part_usage 'subcomponents' : 'MassedThing' multiplicity)
      (attribute_usage :>> 'totalMass' value))
    (part_usage 'filteredMassThing' :> 'compositeThing'
      (attribute_usage abstract 'minMass' :> 'ISQ::mass')
      (attribute_usage :>> 'totalMass' value))))
~~~
# FORMAT
~~~sysml
package MassRollup {
    private import NumericalFunctions::*;

    part def MassedThing {
        attribute mass :> ISQ::mass;
        attribute totalMass :> ISQ::mass;
    }

    part simpleThing : MassedThing {
        attribute redefines totalMass = mass;
    }

    part compositeThing : MassedThing {
        part subcomponents : MassedThing [*];

        attribute redefines totalMass default = mass + sum(subcomponents.totalMass);
    }

    part filteredMassThing :> compositeThing {
        abstract attribute minMass :> ISQ::mass;

        attribute redefines totalMass = mass + sum(subcomponents.totalMass.?{in p :> ISQ::mass; p > minMass});
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
~~~
# SMG
~~~
(model
  (namespace
    (package 'MassRollup'
      (namespace_import private -> 'NumericalFunctions'[unresolved])
      (part_def 'MassedThing'
        (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved])
        (attribute_usage composite 'totalMass' :> 'ISQ::mass'[unresolved]))
      (part_usage 'simpleThing' : 'MassRollup::MassedThing'[part_def]
        (attribute_usage composite :>> 'MassRollup::MassedThing::totalMass'[attribute_usage]
          (feature_value (=))))
      (part_usage 'compositeThing' : 'MassRollup::MassedThing'[part_def]
        (part_usage composite 'subcomponents' : 'MassRollup::MassedThing'[part_def]
          (multiplicity_range [*]))
        (attribute_usage composite :>> 'MassRollup::MassedThing::totalMass'[attribute_usage]
          (feature_value (default =))))
      (part_usage 'filteredMassThing' :> 'MassRollup::compositeThing'[part_usage]
        (attribute_usage abstract composite 'minMass' :> 'ISQ::mass'[unresolved])
        (attribute_usage composite :>> ''[attribute_usage]
          (feature_value (=)))))))
~~~
