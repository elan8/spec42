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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "a_3_6_sequences.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 23 16) (end 23 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 25 16) (end 25 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 26 16) (end 26 42))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "640cdf5c64c8e3c07910fc23c2db7e3bea7ad09f669e15a05b9ae70d9c644a89") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "SequencesExecution"))) (kind "package") (name "SequencesExecution") (declared-name "SequencesExecution") (range (start (line 18) (character 0)) (end (line 18) (character 1207))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 23) (character 1)) (end (line 23) (character 25))) (parent (node (document "d0") (qualified-name "SequencesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "Atoms::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 23) (character 16)) (end (line 23) (character 21))))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 24) (character 1)) (end (line 24) (character 46))) (parent (node (document "d0") (qualified-name "SequencesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequencesModelToBeExecuted::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 24) (character 16)) (end (line 24) (character 42))))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::HappensBefore"))) (kind "import") (name "HappensBefore") (declared-name "HappensBefore") (range (start (line 26) (character 1)) (end (line 26) (character 43))) (parent (node (document "d0") (qualified-name "SequencesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensBefore") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 26) (character 16)) (end (line 26) (character 42))))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::MyDry"))) (kind "kermlDecl") (name "MyDry") (declared-name "MyDry") (range (start (line 31) (character 1)) (end (line 31) (character 32))) (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::MyDry_Before_Ship_Link"))) (kind "kermlDecl") (name "MyDry_Before_Ship_Link") (declared-name "MyDry_Before_Ship_Link") (range (start (line 45) (character 1)) (end (line 45) (character 161))) (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::MyManufacture"))) (kind "kermlDecl") (name "MyManufacture") (declared-name "MyManufacture") (range (start (line 53) (character 1)) (end (line 53) (character 399))) (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::MyManufactureStepsPD"))) (kind "kermlDecl") (name "MyManufactureStepsPD") (declared-name "MyManufactureStepsPD") (range (start (line 39) (character 1)) (end (line 39) (character 53))) (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::MyManufactureStepsPDS"))) (kind "kermlDecl") (name "MyManufactureStepsPDS") (declared-name "MyManufactureStepsPDS") (range (start (line 50) (character 1)) (end (line 50) (character 68))) (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::MyPaint"))) (kind "kermlDecl") (name "MyPaint") (declared-name "MyPaint") (range (start (line 29) (character 1)) (end (line 29) (character 36))) (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::MyPaint_Before_Dry_Link"))) (kind "kermlDecl") (name "MyPaint_Before_Dry_Link") (declared-name "MyPaint_Before_Dry_Link") (range (start (line 34) (character 1)) (end (line 34) (character 163))) (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::MyShip"))) (kind "kermlDecl") (name "MyShip") (declared-name "MyShip") (range (start (line 42) (character 1)) (end (line 42) (character 34))) (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (range (start (line 25) (character 1)) (end (line 25) (character 40))) (parent (node (document "d0") (qualified-name "SequencesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 25) (character 16)) (end (line 25) (character 39))))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::_atom"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 28) (character 1)) (end (line 28) (character 8))) (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::_atom#metadata_keyword"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 30) (character 1)) (end (line 30) (character 8))) (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::_atom#metadata_keyword2"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 33) (character 1)) (end (line 33) (character 8))) (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::_atom#metadata_keyword3"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 41) (character 1)) (end (line 41) (character 8))) (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::_atom#metadata_keyword4"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 44) (character 1)) (end (line 44) (character 8))) (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::_atom#metadata_keyword5"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 52) (character 1)) (end (line 52) (character 8))) (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesModelToBeExecuted"))) (kind "package") (name "SequencesModelToBeExecuted") (declared-name "SequencesModelToBeExecuted") (range (start (line 1) (character 0)) (end (line 1) (character 308))))
    (element (id (node (document "d0") (qualified-name "SequencesModelToBeExecuted::Dry"))) (kind "kermlDecl") (name "Dry") (declared-name "Dry") (range (start (line 14) (character 1)) (end (line 14) (character 14))) (parent (node (document "d0") (qualified-name "SequencesModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "SequencesModelToBeExecuted::Manufacture"))) (kind "kermlDecl") (name "Manufacture") (declared-name "Manufacture") (range (start (line 6) (character 1)) (end (line 6) (character 205))) (parent (node (document "d0") (qualified-name "SequencesModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "SequencesModelToBeExecuted::Paint"))) (kind "kermlDecl") (name "Paint") (declared-name "Paint") (range (start (line 13) (character 1)) (end (line 13) (character 16))) (parent (node (document "d0") (qualified-name "SequencesModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "SequencesModelToBeExecuted::Ship"))) (kind "kermlDecl") (name "Ship") (declared-name "Ship") (range (start (line 15) (character 1)) (end (line 15) (character 15))) (parent (node (document "d0") (qualified-name "SequencesModelToBeExecuted"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "SequencesExecution::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Atoms::*") (range (start (line 23) (character 16)) (end (line 23) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SequencesExecution::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SequencesModelToBeExecuted::*") (range (start (line 24) (character 16)) (end (line 24) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SequencesModelToBeExecuted")))))
    (reference (id (source (node (document "d0") (qualified-name "SequencesExecution::HappensBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensBefore") (range (start (line 26) (character 16)) (end (line 26) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SequencesExecution::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (range (start (line 25) (character 16)) (end (line 25) (character 39))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
