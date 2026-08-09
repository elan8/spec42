# META
~~~ini
description=KerML KerML Spec Annex A: A-3-6-Sequences
type=file
~~~
# SOURCE
~~~kerml

package SequencesModelToBeExecuted {
	doc
	/* 
	 */

	behavior Manufacture {
		step paint : Paint [1];
		step dry : Dry [*];
		succession p_before_d first [1] paint then [1] dry;
		step ship : Ship [*];
		succession d_before_s first [1] dry then [1] ship;
	}
	behavior Paint;
	behavior Dry;
	behavior Ship;
}

package SequencesExecution {
	doc
	/* 
	 */

	private import Atoms::*;
	private import SequencesModelToBeExecuted::*;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensBefore;

	#atom
	behavior MyPaint specializes Paint;
	#atom
	behavior MyDry specializes Dry;

	#atom
	assoc MyPaint_Before_Dry_Link specializes HappensBefore {
		end feature redefines earlierOccurrence : MyPaint;
		end feature redefines laterOccurrence : MyDry;
	}

	behavior MyManufactureStepsPD unions MyPaint, MyDry;

	#atom
	behavior MyShip specializes Ship;

	#atom
	assoc MyDry_Before_Ship_Link specializes HappensBefore {
		end feature redefines earlierOccurrence : MyDry;
		end feature redefines laterOccurrence : MyShip;
	}

	behavior MyManufactureStepsPDS unions MyManufactureStepsPD, MyShip;

	#atom
	behavior MyManufacture specializes Manufacture {
		feature redefines timeEnclosedOccurrences : MyManufactureStepsPDS [3];
		step redefines paint : MyPaint;
		step redefines dry : MyDry [1];
		succession redefines p_before_d : MyPaint_Before_Dry_Link [1] first paint then dry;
		step redefines ship : MyShip [1];
		succession redefines d_before_s : MyDry_Before_Ship_Link [1] first dry then ship;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwBehavior,Ident,OpenCurly,
KwStep,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwStep,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwSuccession,Ident,KwFirst,OpenSquare,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwStep,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwSuccession,Ident,KwFirst,OpenSquare,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
CloseCurly,
KwBehavior,Ident,Semicolon,
KwBehavior,Ident,Semicolon,
KwBehavior,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
Hash,Ident,
KwBehavior,Ident,KwSpecializes,Ident,Semicolon,
Hash,Ident,
KwBehavior,Ident,KwSpecializes,Ident,Semicolon,
Hash,Ident,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwBehavior,Ident,KwUnions,Ident,Comma,Ident,Semicolon,
Hash,Ident,
KwBehavior,Ident,KwSpecializes,Ident,Semicolon,
Hash,Ident,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwBehavior,Ident,KwUnions,Ident,Comma,Ident,Semicolon,
Hash,Ident,
KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwStep,KwRedefines,Ident,Colon,Ident,Semicolon,
KwStep,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwSuccession,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwFirst,Ident,KwThen,Ident,Semicolon,
KwStep,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwSuccession,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwFirst,Ident,KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'SequencesModelToBeExecuted'
    (documentation)
    (behavior_def
      (step_def)
      (step_def)
      (succession_def 'p_before_d'
        (connector_end)
        (connector_end))
      (step_def)
      (succession_def 'd_before_s'
        (connector_end)
        (connector_end)))
    (behavior_def)
    (behavior_def)
    (behavior_def))
  (package_def 'SequencesExecution'
    (documentation)
    (import_decl private 'Atoms::*')
    (import_decl private 'SequencesModelToBeExecuted::*')
    (import_decl private 'Occurrences::Occurrence')
    (import_decl private 'Occurrences::HappensBefore')
    (behavior_def)
    (behavior_def)
    (association_def #'atom' 'MyPaint_Before_Dry_Link' :> 'HappensBefore'
      (feature_def end :>> 'earlierOccurrence' : 'MyPaint')
      (feature_def end :>> 'laterOccurrence' : 'MyDry'))
    (behavior_def)
    (behavior_def)
    (association_def #'atom' 'MyDry_Before_Ship_Link' :> 'HappensBefore'
      (feature_def end :>> 'earlierOccurrence' : 'MyDry')
      (feature_def end :>> 'laterOccurrence' : 'MyShip'))
    (behavior_def)
    (behavior_def
      (feature_def :>> 'timeEnclosedOccurrences' : 'MyManufactureStepsPDS' multiplicity)
      (step_def)
      (step_def)
      (malformed)
      (succession_as_usage
        (connector_end)
        (connector_end))
      (step_def)
      (malformed)
      (succession_as_usage
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml

package SequencesModelToBeExecuted {
	doc
	/* 
	 */

	behavior Manufacture {
		step paint : Paint [1];
		step dry : Dry [*];
		succession p_before_d first [1] paint then [1] dry;
		step ship : Ship [*];
		succession d_before_s first [1] dry then [1] ship;
	}
	behavior Paint;
	behavior Dry;
	behavior Ship;
}

package SequencesExecution {
	doc
	/* 
	 */

	private import Atoms::*;
	private import SequencesModelToBeExecuted::*;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensBefore;

	#atom
	behavior MyPaint specializes Paint;
	#atom
	behavior MyDry specializes Dry;

	#atom
	assoc MyPaint_Before_Dry_Link specializes HappensBefore {
		end feature redefines earlierOccurrence : MyPaint;
		end feature redefines laterOccurrence : MyDry;
	}

	behavior MyManufactureStepsPD unions MyPaint, MyDry;

	#atom
	behavior MyShip specializes Ship;

	#atom
	assoc MyDry_Before_Ship_Link specializes HappensBefore {
		end feature redefines earlierOccurrence : MyDry;
		end feature redefines laterOccurrence : MyShip;
	}

	behavior MyManufactureStepsPDS unions MyManufactureStepsPD, MyShip;

	#atom
	behavior MyManufacture specializes Manufacture {
		feature redefines timeEnclosedOccurrences : MyManufactureStepsPDS [3];
		step redefines paint : MyPaint;
		step redefines dry : MyDry [1];
		succession redefines p_before_d : MyPaint_Before_Dry_Link [1] first paint then dry;
		step redefines ship : MyShip [1];
		succession redefines d_before_s : MyDry_Before_Ship_Link [1] first dry then ship;
	}
}
~~~
# EXPECTED
~~~
parse.expected_keyword_to
parse.expected_keyword_to
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'timeEnclosedOccurrences'
~~~
# PROBLEMS
~~~
parse.expected_keyword_to
parse.expected_keyword_to
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'timeEnclosedOccurrences'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "SequencesExecution"))) (name "SequencesExecution") (declared-name "SequencesExecution")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "SequencesExecution::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SequencesExecution::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SequencesExecution::HappensBefore"))) (name "HappensBefore") (declared-name "HappensBefore"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "SequencesExecution::MyDry"))) (name "MyDry") (declared-name "MyDry"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "SequencesExecution::MyDry_Before_Ship_Link"))) (name "MyDry_Before_Ship_Link") (declared-name "MyDry_Before_Ship_Link"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "SequencesExecution::MyManufacture"))) (name "MyManufacture") (declared-name "MyManufacture"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "SequencesExecution::MyManufactureStepsPD"))) (name "MyManufactureStepsPD") (declared-name "MyManufactureStepsPD"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "SequencesExecution::MyManufactureStepsPDS"))) (name "MyManufactureStepsPDS") (declared-name "MyManufactureStepsPDS"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "SequencesExecution::MyPaint"))) (name "MyPaint") (declared-name "MyPaint"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "SequencesExecution::MyPaint_Before_Dry_Link"))) (name "MyPaint_Before_Dry_Link") (declared-name "MyPaint_Before_Dry_Link"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "SequencesExecution::MyShip"))) (name "MyShip") (declared-name "MyShip"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SequencesExecution::Occurrence"))) (name "Occurrence") (declared-name "Occurrence"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "SequencesExecution::_atom"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "SequencesExecution::_atom#metadata_keyword"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "SequencesExecution::_atom#metadata_keyword2"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "SequencesExecution::_atom#metadata_keyword3"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "SequencesExecution::_atom#metadata_keyword4"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "SequencesExecution::_atom#metadata_keyword5"))) (name "atom") (declared-name "atom"))
      )
    )
    (element (kind "package") (id (node (document "d0") (qualified-name "SequencesModelToBeExecuted"))) (name "SequencesModelToBeExecuted") (declared-name "SequencesModelToBeExecuted")
      (contains
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "SequencesModelToBeExecuted::Dry"))) (name "Dry") (declared-name "Dry"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "SequencesModelToBeExecuted::Manufacture"))) (name "Manufacture") (declared-name "Manufacture"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "SequencesModelToBeExecuted::Paint"))) (name "Paint") (declared-name "Paint"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "SequencesModelToBeExecuted::Ship"))) (name "Ship") (declared-name "Ship"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SequencesExecution::_atom"))) (to (node (document "d0") (qualified-name "SequencesExecution"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SequencesExecution::_atom#metadata_keyword"))) (to (node (document "d0") (qualified-name "SequencesExecution"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SequencesExecution::_atom#metadata_keyword2"))) (to (node (document "d0") (qualified-name "SequencesExecution"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SequencesExecution::_atom#metadata_keyword3"))) (to (node (document "d0") (qualified-name "SequencesExecution"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SequencesExecution::_atom#metadata_keyword4"))) (to (node (document "d0") (qualified-name "SequencesExecution"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SequencesExecution::_atom#metadata_keyword5"))) (to (node (document "d0") (qualified-name "SequencesExecution"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
