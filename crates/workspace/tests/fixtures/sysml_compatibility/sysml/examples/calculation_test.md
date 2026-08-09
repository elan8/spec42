# META
~~~ini
description=SysML Example (Simple Tests): CalculationTest
type=file
~~~
# SOURCE
~~~sysml
package CalculationExample {
	private import ISQ::*;
	private import NumericalFunctions::*;
	
	part def VehiclePart {
		attribute m : MassValue;
	}
	
	part def Vehicle :> VehiclePart;
	
	part vehicle : Vehicle {		
		part eng : VehiclePart;		
		part trans : VehiclePart;
		attribute ::> m = ms.totalMass;
	}
	
	calc def MassSum {
		in partMasses : MassValue[0..*];
		return totalMass : MassValue = sum(partMasses);
	}
	
	calc ms: MassSum {
		in partMasses = (vehicle.eng.m, vehicle.trans.m);
		return totalMass;
	}
	
	part vehicles[*] = (vehicle, vehicle);
	attribute masses1[*] = (vehicles as VehiclePart).m;
	attribute masses2[*] = (vehicles as vehicle).m;
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonColonGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwReturn,Ident,Colon,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,
KwCalc,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,OpenParen,Ident,Dot,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Dot,Ident,CloseParen,Semicolon,
KwReturn,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenSquare,Star,CloseSquare,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,
KwAttribute,Ident,OpenSquare,Star,CloseSquare,Eq,OpenParen,Ident,KwAs,Ident,CloseParen,Dot,Ident,Semicolon,
KwAttribute,Ident,OpenSquare,Star,CloseSquare,Eq,OpenParen,Ident,KwAs,Ident,CloseParen,Dot,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'CalculationExample'
    (import_decl private 'ISQ::*')
    (import_decl private 'NumericalFunctions::*')
    (part_def 'VehiclePart'
      (attribute_usage 'm' : 'MassValue'))
    (part_def 'Vehicle' :> 'VehiclePart')
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'eng' : 'VehiclePart')
      (part_usage 'trans' : 'VehiclePart')
      (attribute_usage references 'm' value))
    (calc_def 'MassSum'
      (default_ref_usage in 'partMasses' : 'MassValue' multiplicity)
      (return_member))
    (calc_usage 'ms' : 'MassSum'
      (default_ref_usage in 'partMasses' value)
      (return_member))
    (part_usage 'vehicles' multiplicity value)
    (attribute_usage 'masses1' multiplicity value)
    (attribute_usage 'masses2' multiplicity value)))
~~~
# FORMAT
~~~sysml
package CalculationExample {
    private import ISQ::*;
    private import NumericalFunctions::*;

    part def VehiclePart {
        attribute m : MassValue;
    }

    part def Vehicle :> VehiclePart;

    part vehicle : Vehicle {
        part eng : VehiclePart;
        part trans : VehiclePart;
        attribute ::> m = ms.totalMass;
    }

    calc def MassSum {
        in partMasses : MassValue[0..*];
        return totalMass : MassValue = sum(partMasses);
    }

    calc ms: MassSum {
        in partMasses = (vehicle.eng.m, vehicle.trans.m);
        return totalMass;
    }

    part vehicles[*] = (vehicle, vehicle);
    attribute masses1[*] = (vehicles as VehiclePart).m;
    attribute masses2[*] = (vehicles as vehicle).m;
}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "CalculationExample"))) (name "CalculationExample") (declared-name "CalculationExample")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "CalculationExample::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "CalculationExample::*#import"))) (name "*") (declared-name "*"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "CalculationExample::MassSum"))) (name "MassSum") (declared-name "MassSum")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "CalculationExample::MassSum::partMasses"))) (name "partMasses") (declared-name "partMasses") (effective (featuring-type (node (document "d0") (qualified-name "CalculationExample::MassSum")))))
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "CalculationExample::MassSum::totalMass"))) (name "totalMass") (declared-name "totalMass") (effective (featuring-type (node (document "d0") (qualified-name "CalculationExample::MassSum")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "CalculationExample::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "CalculationExample::VehiclePart"))) (name "VehiclePart") (declared-name "VehiclePart") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "CalculationExample::VehiclePart::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "CalculationExample::VehiclePart")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "CalculationExample::masses1"))) (name "masses1") (declared-name "masses1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "m") (children (expression (kind "parenthesized") (children (expression (kind "typeCheck") (reference "VehiclePart") (operator "as") (children (expression (kind "featureReference") (reference "vehicles")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "CalculationExample::masses1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "CalculationExample::masses2"))) (name "masses2") (declared-name "masses2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "m") (children (expression (kind "parenthesized") (children (expression (kind "typeCheck") (reference "vehicle") (operator "as") (children (expression (kind "featureReference") (reference "vehicles")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "CalculationExample::masses2"))) (role feature-value))))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "CalculationExample::ms"))) (name "ms") (declared-name "ms")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "CalculationExample::ms::partMasses"))) (name "partMasses") (declared-name "partMasses") (effective (featuring-type (node (document "d0") (qualified-name "CalculationExample::ms")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "CalculationExample::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "CalculationExample::vehicle::eng"))) (name "eng") (declared-name "eng") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CalculationExample::Vehicle")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "CalculationExample::vehicle::trans"))) (name "trans") (declared-name "trans") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CalculationExample::Vehicle")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "CalculationExample::vehicles"))) (name "vehicles") (declared-name "vehicles") (declared (properties (ordered false)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored)) (feature-value (kind bound) (expression (kind "tuple") (children (expression (kind "featureReference") (reference "vehicle")) (expression (kind "featureReference") (reference "vehicle")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "CalculationExample::vehicles"))) (role feature-value))))
      )
    )
  )
  (relationships
    (specializes (status resolved) (from (node (document "d0") (qualified-name "CalculationExample::Vehicle"))) (to (node (document "d0") (qualified-name "CalculationExample::VehiclePart"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CalculationExample::vehicle"))) (to (node (document "d0") (qualified-name "CalculationExample::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CalculationExample::vehicle::eng"))) (to (node (document "d0") (qualified-name "CalculationExample::VehiclePart"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CalculationExample::vehicle::trans"))) (to (node (document "d0") (qualified-name "CalculationExample::VehiclePart"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/calculation_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 1) (end 2 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 2) (end 5 26))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 13 2) (end 13 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 2) (end 17 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 2) (end 18 49))
      )
    )
  )
)
~~~
