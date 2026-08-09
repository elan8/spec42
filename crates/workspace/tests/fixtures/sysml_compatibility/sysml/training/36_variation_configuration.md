# META
~~~ini
description=SysML Training 36 (Variability): Variation Configuration
type=file
~~~
# SOURCE
~~~sysml
package 'Variation Configuration' {
	private import 'Variation Usages'::*;
	
	part vehicle4Cyl :> vehicleFamily {
		part redefines engine = engine::'4cylEngine';
		part redefines transmission = transmission::manualTransmission;
	}
	
	part vehicle6Cyl :> vehicleFamily {
		part redefines engine = engine::'6cylEngine';
		part redefines transmission = transmission::manualTransmission;
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwPart,KwRedefines,Ident,Eq,Ident,ColonColon,UnrestrictedName,Semicolon,
KwPart,KwRedefines,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwPart,KwRedefines,Ident,Eq,Ident,ColonColon,UnrestrictedName,Semicolon,
KwPart,KwRedefines,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Variation Configuration''
    (import_decl private ''Variation Usages'::*')
    (part_usage 'vehicle4Cyl' :> 'vehicleFamily'
      (part_usage :>> 'engine' value)
      (part_usage :>> 'transmission' value))
    (part_usage 'vehicle6Cyl' :> 'vehicleFamily'
      (part_usage :>> 'engine' value)
      (part_usage :>> 'transmission' value))))
~~~
# FORMAT
~~~sysml
package 'Variation Configuration' {
    private import 'Variation Usages'::*;

    part vehicle4Cyl :> vehicleFamily {
        part redefines engine = engine::'4cylEngine';
        part redefines transmission = transmission::manualTransmission;
    }

    part vehicle6Cyl :> vehicleFamily {
        part redefines engine = engine::'6cylEngine';
        part redefines transmission = transmission::manualTransmission;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'vehicleFamily'
semantic.unresolved_name 'engine'
semantic.unresolved_name 'transmission'
semantic.unresolved_name 'vehicleFamily'
semantic.unresolved_name 'engine'
semantic.unresolved_name 'transmission'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'vehicleFamily'
semantic.unresolved_name 'engine'
semantic.unresolved_name 'transmission'
semantic.unresolved_name 'vehicleFamily'
semantic.unresolved_name 'engine'
semantic.unresolved_name 'transmission'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Variation Configuration'
      (namespace_import private -> 'Variation Usages'[unresolved])
      (part_usage 'vehicle4Cyl' :> 'vehicleFamily'[unresolved]
        (part_usage composite :>> 'engine'[unresolved]
          (feature_value (=)))
        (part_usage composite :>> 'transmission'[unresolved]
          (feature_value (=))))
      (part_usage 'vehicle6Cyl' :> 'vehicleFamily'[unresolved]
        (part_usage composite :>> 'engine'[unresolved]
          (feature_value (=)))
        (part_usage composite :>> 'transmission'[unresolved]
          (feature_value (=)))))))
~~~
