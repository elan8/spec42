# META
~~~ini
description=SysML Training 41 (Language Extension): Model Library Example
type=file
~~~
# SOURCE
~~~sysml
library package 'Model Library Example' {
	private import ScalarValues::Real;
	private import RiskMetadata::Level;
	
	abstract occurrence def Situation;
	
	abstract occurrence situations : Situation[*] nonunique;
	
	abstract occurrence def Cause {
		attribute probability : Real;
	}
	
	abstract occurrence causes : Cause[*] nonunique :> situations;
	
	abstract occurrence def Failure {
		attribute severity : Level;
	}
	
	abstract occurrence failures : Failure[*] nonunique :> situations;
	
	abstract connection def Causation :> Occurrences::HappensBefore {
		end [*] ref cause : Situation;
		end [*] ref effect : Situation;
	}
	
	abstract connection causations : Causation[*] nonunique;
	
	item def Scenario {
		occurrence :>> situations;
		occurrence :>> causes :> situations;
		occurrence :>> failures :> situations;
	}
	
	item scenarios : Scenario[*] nonunique;
}
~~~
# TOKENS
~~~zig
KwLibrary,KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwOccurrence,KwDef,Ident,Semicolon,
KwAbstract,KwOccurrence,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
KwAbstract,KwOccurrence,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwOccurrence,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAbstract,KwOccurrence,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwOccurrence,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAbstract,KwConnection,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwEnd,OpenSquare,Star,CloseSquare,KwRef,Ident,Colon,Ident,Semicolon,
KwEnd,OpenSquare,Star,CloseSquare,KwRef,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwConnection,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
KwItem,KwDef,Ident,OpenCurly,
KwOccurrence,ColonGtGt,Ident,Semicolon,
KwOccurrence,ColonGtGt,Ident,ColonGt,Ident,Semicolon,
KwOccurrence,ColonGtGt,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwItem,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (library_package_def ''Model Library Example''
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'RiskMetadata::Level')
    (occurrence_def abstract 'Situation')
    (occurrence_usage abstract 'situations' : 'Situation' multiplicity nonunique)
    (occurrence_def abstract 'Cause'
      (attribute_usage 'probability' : 'Real'))
    (occurrence_usage abstract 'causes' : 'Cause' :> 'situations' multiplicity nonunique)
    (occurrence_def abstract 'Failure'
      (attribute_usage 'severity' : 'Level'))
    (occurrence_usage abstract 'failures' : 'Failure' :> 'situations' multiplicity nonunique)
    (connection_def abstract 'Causation' :> 'Occurrences::HappensBefore'
      (interface_end end 'cause' : 'Situation' multiplicity)
      (interface_end end 'effect' : 'Situation' multiplicity))
    (connection_usage 'Causation' 'causations' multiplicity)
    (item_def 'Scenario'
      (occurrence_usage :>> 'situations')
      (occurrence_usage :>> 'causes' :> 'situations')
      (occurrence_usage :>> 'failures' :> 'situations'))
    (item_usage 'scenarios' : 'Scenario' multiplicity nonunique)))
~~~
# FORMAT
~~~sysml
library package 'Model Library Example' {
    private import ScalarValues::Real;
    private import RiskMetadata::Level;

    abstract occurrence def Situation;

    abstract occurrence situations : Situation [*] nonunique;

    abstract occurrence def Cause {
        attribute probability : Real;
    }

    abstract occurrence causes : Cause :> situations [*] nonunique;

    abstract occurrence def Failure {
        attribute severity : Level;
    }

    abstract occurrence failures : Failure :> situations [*] nonunique;

    abstract connection def Causation :> Occurrences::HappensBefore {
        end [*] cause : Situation;
        end [*] effect : Situation;
    }

    abstract connection causations : Causation [*];

    item def Scenario {
        occurrence :>> situations;
        occurrence :>> causes :> situations;
        occurrence :>> failures :> situations;
    }

    item scenarios : Scenario [*] nonunique;
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Level'
semantic.unresolved_name 'Occurrences::HappensBefore'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Level'
semantic.unresolved_name 'Occurrences::HappensBefore'
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'Model Library Example'
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (membership_import private -> 'RiskMetadata::Level'[unresolved])
      (occurrence_def abstract 'Situation')
      (occurrence_usage abstract 'situations' : 'Model Library Example::Situation'[occurrence_def]
        (multiplicity_range [*]))
      (occurrence_def abstract 'Cause'
        (attribute_usage composite 'probability' : 'Real'[unresolved]))
      (occurrence_usage abstract 'causes' : 'Model Library Example::Cause'[occurrence_def] :> 'Model Library Example::situations'[occurrence_usage]
        (multiplicity_range [*]))
      (occurrence_def abstract 'Failure'
        (attribute_usage composite 'severity' : 'Level'[unresolved]))
      (occurrence_usage abstract 'failures' : 'Model Library Example::Failure'[occurrence_def] :> 'Model Library Example::situations'[occurrence_usage]
        (multiplicity_range [*]))
      (connection_def abstract 'Causation' :> 'Occurrences::HappensBefore'[unresolved]
        (port_usage end 'cause' : 'Model Library Example::Situation'[occurrence_def]
          (multiplicity_range [*]))
        (port_usage end 'effect' : 'Model Library Example::Situation'[occurrence_def]
          (multiplicity_range [*])))
      (connection_usage abstract 'causations' : 'Model Library Example::Causation'[connection_def]
        (multiplicity_range [*]))
      (item_def 'Scenario'
        (occurrence_usage composite :>> 'Model Library Example::situations'[occurrence_usage])
        (occurrence_usage composite :>> 'Model Library Example::causes'[occurrence_usage] :> ''[occurrence_usage])
        (occurrence_usage composite :>> 'Model Library Example::failures'[occurrence_usage] :> ''[occurrence_usage]))
      (item_usage 'scenarios' : 'Model Library Example::Scenario'[item_def]
        (multiplicity_range [*])))))
~~~
