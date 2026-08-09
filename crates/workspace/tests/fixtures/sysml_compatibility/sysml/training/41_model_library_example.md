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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Model Library Example"))) (name "Model Library Example") (declared-name "Model Library Example")
      (contains
        (element (kind "connection def") (id (node (document "d0") (qualified-name "Model Library Example::Causation"))) (name "Causation") (declared-name "Causation")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Model Library Example::Causation::cause"))) (name "cause") (declared-name "cause") (declared (properties (end true)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Model Library Example::Causation")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Model Library Example::Causation::effect"))) (name "effect") (declared-name "effect") (declared (properties (end true)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Model Library Example::Causation")))))
          )
        )
        (element (kind "occurrence def") (id (node (document "d0") (qualified-name "Model Library Example::Cause"))) (name "Cause") (declared-name "Cause") (declared (properties (abstract true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Model Library Example::Cause::probability"))) (name "probability") (declared-name "probability") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Model Library Example::Cause")))))
          )
        )
        (element (kind "occurrence def") (id (node (document "d0") (qualified-name "Model Library Example::Failure"))) (name "Failure") (declared-name "Failure") (declared (properties (abstract true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Model Library Example::Failure::severity"))) (name "severity") (declared-name "severity") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Model Library Example::Failure")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Model Library Example::Level"))) (name "Level") (declared-name "Level"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Model Library Example::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Model Library Example::Scenario"))) (name "Scenario") (declared-name "Scenario")
          (contains
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "Model Library Example::Scenario::"))) (name "") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Model Library Example::Scenario")))))
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence"))) (name "") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Model Library Example::Scenario")))))
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence2"))) (name "") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Model Library Example::Scenario")))))
          )
        )
        (element (kind "occurrence def") (id (node (document "d0") (qualified-name "Model Library Example::Situation"))) (name "Situation") (declared-name "Situation") (declared (properties (abstract true))))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "Model Library Example::causations"))) (name "causations") (declared-name "causations"))
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "Model Library Example::causes"))) (name "causes") (declared-name "causes") (declared (properties (abstract true))))
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "Model Library Example::failures"))) (name "failures") (declared-name "failures") (declared (properties (abstract true))))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Model Library Example::scenarios"))) (name "scenarios") (declared-name "scenarios"))
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "Model Library Example::situations"))) (name "situations") (declared-name "situations") (declared (properties (abstract true))))
      )
    )
  )
  (relationships
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Model Library Example::causations"))) (to (node (document "d0") (qualified-name "Model Library Example::Causation"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Model Library Example::scenarios"))) (to (node (document "d0") (qualified-name "Model Library Example::Scenario"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Model Library Example::causes"))) (to (node (document "d0") (qualified-name "Model Library Example::situations"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Model Library Example::failures"))) (to (node (document "d0") (qualified-name "Model Library Example::situations"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Model Library Example::Causation::cause"))) (to (node (document "d0") (qualified-name "Model Library Example::Situation"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Model Library Example::Causation::effect"))) (to (node (document "d0") (qualified-name "Model Library Example::Situation"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Model Library Example::causes"))) (to (node (document "d0") (qualified-name "Model Library Example::Cause"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Model Library Example::failures"))) (to (node (document "d0") (qualified-name "Model Library Example::Failure"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Model Library Example::situations"))) (to (node (document "d0") (qualified-name "Model Library Example::Situation"))))
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
  (document "sysml/training/41_model_library_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 2) (end 9 31))
      )
      (diagnostic
        (severity error)
        (code "subsetting_type_incompatible")
        (source "semantic")
        (range (start 12 21) (end 12 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 2) (end 15 29))
      )
      (diagnostic
        (severity error)
        (code "subsetting_type_incompatible")
        (source "semantic")
        (range (start 18 21) (end 18 67))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 20 1) (end 20 136))
      )
      (diagnostic
        (severity warning)
        (code "incompatible_specializes_kind")
        (source "semantic")
        (range (start 25 1) (end 25 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 28 13) (end 28 28))
      )
    )
  )
)
~~~
