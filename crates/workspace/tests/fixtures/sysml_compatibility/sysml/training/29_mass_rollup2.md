# META
~~~ini
description=SysML Training 29 (Expressions): MassRollup2
type=file
~~~
# SOURCE
~~~sysml
package MassRollup2 {
	private import NumericalFunctions::*;
	
	part def MassedThing {
		attribute simpleMass :> ISQ::mass; 
		attribute totalMass :> ISQ::mass default simpleMass;
	}
	
	part compositeThing : MassedThing {
		part subcomponents: MassedThing[*];		
		attribute :>> totalMass default
			simpleMass + sum(subcomponents.totalMass); 
	}
	
	part filteredMassThing :> compositeThing {
		attribute minMass :> ISQ::mass;		
		attribute :>> totalMass =
			simpleMass + sum(subcomponents.totalMass.?{in p:>ISQ::mass; p >= minMass});
	}

}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,KwDefault,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,KwDefault,
Ident,Plus,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,
Ident,Plus,Ident,OpenParen,Ident,Dot,Ident,DotQuestion,OpenCurly,KwIn,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,Ident,GtEq,Ident,CloseCurly,CloseParen,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'MassRollup2'
    (import_decl private 'NumericalFunctions::*')
    (part_def 'MassedThing'
      (attribute_usage 'simpleMass' :> 'ISQ::mass')
      (attribute_usage 'totalMass' :> 'ISQ::mass' value))
    (part_usage 'compositeThing' : 'MassedThing'
      (part_usage 'subcomponents' : 'MassedThing' multiplicity)
      (attribute_usage :>> 'totalMass' value))
    (part_usage 'filteredMassThing' :> 'compositeThing'
      (attribute_usage 'minMass' :> 'ISQ::mass')
      (attribute_usage :>> 'totalMass' value))))
~~~
# FORMAT
~~~sysml
package MassRollup2 {
    private import NumericalFunctions::*;

    part def MassedThing {
        attribute simpleMass :> ISQ::mass;
        attribute totalMass :> ISQ::mass default = simpleMass;
    }

    part compositeThing : MassedThing {
        part subcomponents : MassedThing [*];
        attribute :>> totalMass default = simpleMass + sum(subcomponents.totalMass);
    }

    part filteredMassThing :> compositeThing {
        attribute minMass :> ISQ::mass;
        attribute :>> totalMass = simpleMass + sum(subcomponents.totalMass.?{in p:>ISQ::mass; p >= minMass});
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
    (package 'MassRollup2'
      (namespace_import private -> 'NumericalFunctions'[unresolved])
      (part_def 'MassedThing'
        (attribute_usage composite 'simpleMass' :> 'ISQ::mass'[unresolved])
        (attribute_usage composite 'totalMass' :> 'ISQ::mass'[unresolved]
          (feature_value (default =))))
      (part_usage 'compositeThing' : 'MassRollup2::MassedThing'[part_def]
        (part_usage composite 'subcomponents' : 'MassRollup2::MassedThing'[part_def]
          (multiplicity_range [*]))
        (attribute_usage composite :>> 'MassRollup2::MassedThing::totalMass'[attribute_usage]
          (feature_value (default =))))
      (part_usage 'filteredMassThing' :> 'MassRollup2::compositeThing'[part_usage]
        (attribute_usage composite 'minMass' :> 'ISQ::mass'[unresolved])
        (attribute_usage composite :>> ''[attribute_usage]
          (feature_value (=)))))))
~~~
